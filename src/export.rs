use crate::TestOutcome;

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

use crate::semihosting_ext::{syscall_readonly, ParamRegR};
pub use heapless::Vec;

#[derive(Debug, Copy, Clone, serde::Serialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Test {
    pub name: &'static str,
    #[serde(skip)]
    pub function: fn() -> !,
    pub should_panic: bool,
    pub ignored: bool,
}

pub fn run_tests(tests: &mut [Test]) -> ! {
    for test in tests.iter_mut() {
        test.name = &test.name[test.name.find("::").unwrap() + 2..];
    }

    let mut args = &semihosting::experimental::env::args::<1024>().unwrap(); //TODO: handle error
    let command = match args.next() {
        Some(c) => c.unwrap(),
        None => {
            error!("Received no arguments via semihosting. Please communicate with the target with the embedded-test runner.");
            semihosting::process::abort();
        }
    };

    match command {
        "list" => {
            info!("tests available: {:?}", tests);
            let mut buf = [0u8; 1024];
            let size = serde_json_core::to_slice(&tests, &mut buf).unwrap();
            let args = [ParamRegR::ptr(buf.as_ptr()), ParamRegR::usize(size)];
            let ret = unsafe { syscall_readonly(0x100, ParamRegR::block(&args)) };
            if ret.usize() != 0 {
                error!("syscall failed: {}", ret.usize());
                semihosting::process::abort();
            }

            semihosting::process::exit(0);
        }
        "run" => {
            let test_name = args.next().unwrap().unwrap();
            let test = tests.iter_mut().find(|t| t.name == test_name).unwrap();
            info!("Running test: {:?}", test);
            (test.function)();
        }
        _ => {
            error!("Unknown command: {}", command);
            semihosting::process::abort();
        }
    }
}

pub fn check_outcome<T: TestOutcome>(outcome: T) -> ! {
    if outcome.is_success() {
        info!("Test exited with () or Ok(..)");
        semihosting::process::exit(0);
    } else {
        info!("Test exited with Err(..): {:?}", outcome);
        semihosting::process::abort();
    }
}
