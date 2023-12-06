extern crate libtest_mimic;

use std::env;
use std::{thread, time};
use std::fs::File;
use std::time::{Duration, Instant};
use libtest_mimic::{Arguments, Trial, Failed};
use log::*;
use probe_rs::{BreakpointCause, Core, CoreStatus, HaltReason, Lister, MemoryInterface, Permissions, SemihostingCommand, Session};
use probe_rs::flashing::{DownloadOptions, IdfOptions};


use anyhow::{bail, Context, Result};


/// Creates a probe-rs session using default settings
fn create_session() -> Result<Session> {
    // Get a list of all available debug probes.
    let lister = Lister::new();
    let probes =  lister.list_all();
    let probe =probes.first().expect("No probe found");
    let probe = lister.open(probe)?;

    let target = "esp32c6"; // TODO: make this configurable
    Ok(probe.attach(target, Permissions::new())?)
}

/// Flashes the chip and resets it, using default settings
fn download(session: &mut Session, elf: &str) -> Result<()>
{
    let mut file =  File::open(&elf).context("failed to open elf")?;

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
    //TODO: Print rtt messages
    core.run()?;

    loop {
        match core.status()? {
            CoreStatus::Halted(HaltReason::Breakpoint(BreakpointCause::Semihosting(s))) => {
                info!("Got semihosting command from target {:?}", s);
                return Ok(s)
            },
            CoreStatus::Halted(r) => bail!("core halted {:?}", r),
            probe_rs::CoreStatus::Running
            | probe_rs::CoreStatus::LockedUp
            | probe_rs::CoreStatus::Sleeping
            | probe_rs::CoreStatus::Unknown => {
        }}

        std::thread::sleep(Duration::from_millis(100));
    }
}


fn run_until_exact_semihosting(core: &mut Core, operation: u32) -> Result<u32>
{
    match run_until_semihosting(core)? {
        SemihostingCommand::ExitSuccess |
        SemihostingCommand::ExitError { .. } => { bail!("Unexpected exit of target at program start")}
        SemihostingCommand::Unknown { operation: op, parameter } => {
            if op == operation {
                Ok(parameter)
            } else {
                bail!("Unexpected semihosting operation: {:x}", operation)
            }
        }
    }
}


/// Asks the target for the tests, and create closures to run the tests later
fn create_tests(core: &mut Core) -> Result<Vec<Trial>>
{
    {
        const SYS_GET_CMDLINE: u32 = 0x15;
        let parameter = run_until_exact_semihosting(core, SYS_GET_CMDLINE)?;
        let mut block: [u32; 2] = [0, 0];
        core.read_32(parameter as u64, &mut block)?;
        let buf_ptr = block[0];
        let buf_size = &mut block[1];

        let msg = b"list\0";
        core.write_8(buf_ptr as u64, msg)?;
        *buf_size = msg.len() as u32 - 1; // String length without zero termination
        core.write_32(parameter as u64, &mut block)?;
        core.write_core_reg(core.registers().get_argument_register(0).unwrap(), 0u32)?;
        // write status = success
        info!("wrote cmdline");
    }


    {
        //TODO: Dedup this struct
        #[derive(Debug, Copy, Clone)]
        #[derive(serde::Deserialize)]
        pub struct Test<'a> {
            pub name: &'a str,
            pub should_error: bool,
            pub ignored: bool,
        }

        const USER_LIST: u32 = 0x100;
        let parameter = run_until_exact_semihosting(core, USER_LIST)?;
        let mut block: [u32; 2] = [0, 0];
        core.read_32(parameter as u64, &mut block)?;
        let buf_ptr = block[0];
        let buf_size = block[1] as usize;
        let mut buf = vec![0u8; buf_size];
        core.read(buf_ptr as u64, &mut buf[..])?;
        let list: Vec<Test> = serde_json::from_slice(&buf[..])?;
        info!("got list: {:?}", list);

        let mut tests = Vec::<Trial>::new();
        for t in &list {
            tests.push(Trial::test(t.name, || { Ok(()) }).with_ignored_flag(t.ignored))
        }
        Ok(tests)
    }
}

fn main() -> Result<()>{
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("test_runner=info")).init();


    // Get all command-line arguments, including the program name
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let elf = args[1].clone(); // get elf (first positional arg).
    info!("Flashing elf file {}", elf);

    // Create an iterator from the remaining arguments, skipping the first argument
    let mut args_for_libtest_mimic = vec![program_name];
    args_for_libtest_mimic.extend( args.into_iter().skip(2));
    let args = Arguments::from_iter(args_for_libtest_mimic);

    let mut session = create_session()?;
    download(&mut session, &elf)?;

    let mut core = session.core(0)?;

    let tests = create_tests(&mut core)?;

    libtest_mimic::run(&args, tests).exit();
}


// Tests

fn check_toph() -> Result<(), Failed> {
    Ok(())
}
fn check_katara() -> Result<(), Failed> {
    Ok(())
}
fn check_sokka() -> Result<(), Failed> {
    Err("Sokka tripped and fell :(".into())
}
fn long_computation() -> Result<(), Failed> {
    thread::sleep(time::Duration::from_secs(1));
    Ok(())
}
fn compile_fail_dummy() -> Result<(), Failed> {
    Ok(())
}