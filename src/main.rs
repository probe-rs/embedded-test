extern crate libtest_mimic;

use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::time::{Duration, Instant};
use libtest_mimic::{Arguments, Trial, Failed};
use log::*;
use probe_rs::{BreakpointCause, Core, CoreStatus, HaltReason, Lister, MemoryInterface, Permissions, SemihostingCommand, Session};
use probe_rs::flashing::{DownloadOptions, IdfOptions};


use anyhow::{bail, Context, Result};
use static_cell::StaticCell;


/// Creates a probe-rs session using default settings
fn create_session() -> Result<Session> {
    // Get a list of all available debug probes.
    let lister = Lister::new();
    let probes = lister.list_all();
    let probe = probes.first().expect("No probe found");
    let probe = lister.open(probe)?;

    let target = "esp32c6"; // TODO: make this configurable
    Ok(probe.attach(target, Permissions::new())?)
}

/// Flashes the chip and resets it, using default settings
fn download(session: &mut Session, elf: &str) -> Result<()>
{
    let mut file = File::open(&elf).context("failed to open elf")?;

    let mut loader = session.target().flash_loader();

    let instant = Instant::now();
    loader.load_idf_data(session, &mut file, IdfOptions::default())?; //TODO: Make configurable
    loader.commit(session, DownloadOptions::new())?;

    // Stop timer.
    let elapsed = instant.elapsed();
    info!(
            "Finished in {}s",
            elapsed.as_millis() as f32 / 1000.0,
        );

    session
        .core(0)?
        .reset_and_halt(Duration::from_millis(100))?;
    Ok(())
}


fn run_until_semihosting(core: &mut Core) -> Result<SemihostingCommand>
{
    const SYS_EXIT_EXTENDED: u32 = 0x20;

    //TODO: Print rtt messages
    core.run()?;

    loop {
        match core.status()? {
            CoreStatus::Halted(HaltReason::Breakpoint(BreakpointCause::Semihosting(SemihostingCommand::Unknown { operation, .. }))) if operation == SYS_EXIT_EXTENDED => {
                debug!("Got SYS_EXIT_EXTENDED. Continuing");
                core.run()?;
            }
            CoreStatus::Halted(HaltReason::Breakpoint(BreakpointCause::Semihosting(s))) => {
                debug!("Got semihosting command from target {:?}", s);
                return Ok(s);
            }
            CoreStatus::Halted(r) => bail!("core halted {:?}", r),
            probe_rs::CoreStatus::Running
            | probe_rs::CoreStatus::LockedUp
            | probe_rs::CoreStatus::Sleeping
            | probe_rs::CoreStatus::Unknown => {}
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}


fn run_until_exact_semihosting(core: &mut Core, operation: u32) -> Result<u32>
{
    match run_until_semihosting(core)? {
        SemihostingCommand::ExitSuccess |
        SemihostingCommand::ExitError { .. } => { bail!("Unexpected exit of target at program start") }
        SemihostingCommand::Unknown { operation: op, parameter } => {
            if op == operation {
                Ok(parameter)
            } else {
                bail!("Unexpected semihosting operation: {:x}", operation)
            }
        }
    }
}


struct Buffer {
    address: u32,
    len: u32,
}

impl Buffer {
    fn from_block_at(core: &mut Core, block_addr: u32) -> Result<Self> {
        let mut block: [u32; 2] = [0, 0];
        core.read_32(block_addr as u64, &mut block)?;
        Ok(Self {
            address: block[0],
            len: block[1],
        })
    }

    fn read(&mut self, core: &mut Core) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; self.len as usize];
        core.read(self.address as u64, &mut buf[..])?;
        Ok(buf)
    }

    // Writes the passed buffer to the target. The buffer must end with \0
    // length written will not include \0.
    fn write_to_block_at(&mut self, core: &mut Core, block_addr: u32, buf: &[u8]) -> Result<()> {
        if buf.len() > self.len as usize {
            bail!("buffer not large enough")
        }
        if *buf.last().unwrap() != 0 {
            bail!("last byte is not 0");
        }
        core.write_8(self.address as u64, buf)?;
        let block: [u32; 2] = [self.address, (buf.len() - 1) as u32];
        core.write_32(block_addr as u64, &block)?;
        Ok(())
    }
}


//TODO: Dedup this struct
#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct Test {
    pub name: String,
    pub should_error: bool,
    pub ignored: bool,
}

/// Asks the target for the tests, and create closures to run the tests later
fn create_tests(core_ref : &'static RefCell<Core<'static>>) -> Result<Vec<Trial>>
{
    let mut core = core_ref.borrow_mut();
    let core = &mut *core;
    // Run target with arg "list", so that it lists all tests
    {
        const SYS_GET_CMDLINE: u32 = 0x15;
        let block_address = run_until_exact_semihosting(core, SYS_GET_CMDLINE)?;
        let mut buf = Buffer::from_block_at(core, block_address)?;
        buf.write_to_block_at(core, block_address, b"list\0")?;

        let reg = core.registers().get_argument_register(0).unwrap();
        core.write_core_reg(reg, 0u32)?;   // write status = success
    }

    // Wait until the target calls the user defined Semihosting Operation and reports the tests
    {
        const USER_LIST: u32 = 0x100;
        let block_address = run_until_exact_semihosting(core, USER_LIST)?;
        let mut buf = Buffer::from_block_at(core, block_address)?;
        let buf = buf.read(core)?;

        let list: Vec<Test> = serde_json::from_slice(&buf[..])?;
        debug!("got list of tests from target: {:?}", list);

        let mut tests = Vec::<Trial>::new();
        for t in &list {
            let test = t.clone();
            tests.push(Trial::test(&t.name, move || {
                let mut core = core_ref.borrow_mut();
                run_test(test, &mut *core)
            }).with_ignored_flag(t.ignored))
        }
        Ok(tests)
    }
}

// Run a single test on the target
fn run_test(test: Test, core: &mut Core) -> core::result::Result<(), Failed> {
    info!("Running test {}", test.name);
    core.reset_and_halt(Duration::from_millis(100))?;

    // Run target with arg "run <testname>"
    {
        const SYS_GET_CMDLINE: u32 = 0x15;
        let block_address = run_until_exact_semihosting(core, SYS_GET_CMDLINE)?;
        let mut buf = Buffer::from_block_at(core, block_address)?;
        let cmd = format!("run {}\0", test.name).into_bytes();
        buf.write_to_block_at(core, block_address, &cmd)?;
        let reg = core.registers().get_argument_register(0).unwrap();
        core.write_core_reg(reg, 0u32)?;   // write status = success
    }

    // Wait on semihosting abort/exit
    match run_until_semihosting(core)? {
        SemihostingCommand::ExitSuccess => {
            info!("Test ok");
            Ok(())
        }
        SemihostingCommand::ExitError { .. } => {
            info!("Test failed");
            Err(Failed::without_message())
        }
        SemihostingCommand::Unknown { operation, parameter } => Err(Failed::from(format!("Expected the target to run the test and exit/error with semihosting. Instead it requested semihosting operation: {} {:x}", operation, parameter)))
    }
}

static SESSION: StaticCell<Session> = StaticCell::new();
static CORE: StaticCell<RefCell<Core<'static>>> = StaticCell::new();

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("embedded_test=info")).init();

    // Get all command-line arguments, including the program name
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let elf = args[1].clone();
    // get elf (first positional arg).
    info!("Flashing elf file {}", elf);

    // Create an iterator from the remaining arguments, skipping the first argument
    let mut args_for_libtest_mimic = vec![program_name];
    args_for_libtest_mimic.extend(args.into_iter().skip(2));
    let args = Arguments::from_iter(args_for_libtest_mimic);

    let session = SESSION.init(create_session()?);
    download(session, &elf)?;
    let core = CORE.init(RefCell::new(session.core(0)?));

    let tests = create_tests(core)?;

    libtest_mimic::run(&args, tests).exit();
}