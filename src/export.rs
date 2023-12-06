
use crate::TestOutcome;


#[cfg(all(feature = "defmt", feature = "log"))]
compile_error!("You may not enable both `defmt` and `log` features.");

#[cfg(all(not(feature = "defmt"), not(feature = "log")))]
compile_error!("You have to enable either the `defmt` or the `log` feature");


/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
pub fn exit() -> ! {
    loop {
        semihosting::process::exit(0);
    }
}

/// Terminates the application and makes a semihosting-capable debug tool exit with error
pub fn abort() -> ! {
    loop {
        semihosting::process::abort();
    }
}

pub use heapless::Vec;
use crate::semihosting_ext::{ParamRegR, syscall_readonly};

#[derive(Debug, Copy, Clone)]
#[derive(serde::Serialize)]
pub struct Test {
    pub name: &'static str,
    //#[musli(skip)]
    #[serde(skip)]
    pub function: fn() -> (),
    pub should_error: bool,
    pub ignored: bool,
}





pub fn run_tests(tests: &mut [Test]) -> !{
    for test in tests.iter_mut() {
        test.name = &test.name[test.name.find("::").unwrap()+2 ..];
    }


    let mut args = &semihosting::experimental::env::args::<1024>().unwrap(); //TODO: handle error
    let command = match args.next() {
        Some(c) => c.unwrap(),
        None => {
            log::error!("Received no arguments via semihosting. Please communicate with the target with the embedded-test runner.");
            semihosting::process::abort();
        }
    };

    match command
    {
        "list" => {
            log::info!("tests available: {:?}", tests);

            let mut buf = [0u8; 1024];
            let size = serde_json_core::to_slice(&tests, &mut buf).unwrap();
            let args = [ParamRegR::ptr(buf.as_ptr()), ParamRegR::usize(size)];
            let ret = unsafe {syscall_readonly(0x100, ParamRegR::block(&args))};
            if ret.usize() != 0 {
                log::error!("syscall failed: {}", ret.usize());
                semihosting::process::abort();
            }

            semihosting::process::exit(0);
        },
        "run" => {
            let test_name = args.next().unwrap().unwrap();
            let test = tests.iter_mut().find(|t| t.name == test_name).unwrap();
            log::info!("Running test: {:?}", test);

            (test.function)(); // TODO: handle outcome

            log::info!("OK");
            semihosting::process::exit(0);
        }
        _ => {
            log::error!("Unknown command: {}", command);
            semihosting::process::abort();
        }
    }
}


/// Fetches the command line arguments from the host via semihosting
pub fn args<const BUF_SIZE: usize>() ->  semihosting::experimental::env::Args<BUF_SIZE> {
    semihosting::experimental::env::args().unwrap() //TODO: handle error
}

#[cfg(feature = "defmt")]
pub fn check_outcome<T: TestOutcome>(outcome: T, should_error: bool) {
    if outcome.is_success() == should_error {
        let note = if should_error {
            defmt::intern!("`#[should_error]` ")
        } else {
            defmt::intern!("")
        };
        defmt::panic!("{}test failed with outcome: {}", note, outcome);
    }
}

#[cfg(feature = "log")]
pub fn check_outcome<T: TestOutcome>(outcome: T, should_error: bool) {
    if outcome.is_success() == should_error {
        let note = if should_error {
            "`#[should_error]` "
        } else {
            ""
        };
        core::panic!("{}test failed with outcome: {:?}", note, outcome);
    }
}