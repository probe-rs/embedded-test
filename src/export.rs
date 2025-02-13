use crate::TestOutcome;

pub use heapless::Vec;

use semihosting::sys::arm_compat::syscall::ParamRegR;
use semihosting::sys::arm_compat::syscall::{syscall_readonly, OperationNumber};

pub fn ensure_linker_file_was_added_to_rustflags() -> ! {
    // Try to access a symbol which we provide in the embedded-test.x linker file.
    // This will trigger a linker error if the linker file has not been added to the rustflags
    extern "C" {
        fn embedded_test_linker_file_not_added_to_rustflags() -> !;
    }
    unsafe { embedded_test_linker_file_not_added_to_rustflags() }
}

// Reexport the embassy stuff
#[cfg(all(feature = "embassy", not(feature = "ariel-os")))]
pub use embassy_executor::task;
#[cfg(all(
    feature = "embassy",
    not(feature = "external-executor"),
    not(feature = "ariel-os")
))]
pub use embassy_executor::Executor; // Please activate the `executor-thread` or `executor-interrupt` feature on the embassy-executor crate (v0.7.x)!

const VERSION: u32 = 1; //Format version of our protocol between probe-rs and target running embedded-test

// Constants ahead that are used by proc macros to calculate the needed json buffer size
pub const JSON_SIZE_HEADER: usize = r#"{"version:12,tests:[]}"#.len();
pub const JSON_SIZE_PER_TEST_WITHOUT_TESTNAME: usize =
    r#"{"name":"","should_panic":false,ignored:false,timeout:1234567890},"#.len();

/// Struct which will be serialized as JSON and sent to probe-rs when it requests the available tests
// NOTE: Update the const's above if you change something here
#[derive(Debug, serde::Serialize)]
struct Tests<'a> {
    pub version: u32,
    pub tests: &'a [Test],
}

/// Struct which describes a single test to probe-rs
// NOTE: Update the const's above if you change something here
#[derive(Debug, serde::Serialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Test {
    pub name: &'static str,
    #[serde(skip)]
    pub function: fn() -> !,
    pub should_panic: bool,
    pub ignored: bool,
    pub timeout: Option<u32>,
}

pub fn run_tests<const JSON_SIZE_TOTAL: usize>(tests: &mut [Test]) -> ! {
    for test in tests.iter_mut() {
        test.name = &test.name[test.name.find("::").unwrap() + 2..];
    }

    let mut args = &semihosting::experimental::env::args::<1024>()
        .expect("Failed to get cmdline via semihosting");
    let command = match args.next() {
        Some(c) => c.expect("command to run contains non-utf8 characters"),
        None => {
            error!("Received no arguments via semihosting. Please communicate with the target with the embedded-test runner.");
            semihosting::process::abort();
        }
    };

    match command {
        "list" => {
            info!("tests available: {:?}", tests);
            let tests = Tests {
                version: VERSION,
                tests,
            };

            let mut buf = [0u8; JSON_SIZE_TOTAL];
            let size = serde_json_core::to_slice(&tests, &mut buf)
                .expect("Buffer to store list of test was too small");
            let args = [ParamRegR::ptr(buf.as_ptr()), ParamRegR::usize(size)];
            let ret = unsafe {
                syscall_readonly(
                    OperationNumber::user_defined(0x100),
                    ParamRegR::block(&args),
                )
            };
            if ret.usize() != 0 {
                error!("syscall failed: {}", ret.usize());
                semihosting::process::abort();
            }

            semihosting::process::exit(0);
        }
        "run" => {
            let test_name = args
                .next()
                .expect("test name missing")
                .expect("test name contains non-utf8 character");
            let test = tests
                .iter_mut()
                .find(|t| t.name == test_name)
                .expect("test to run not found");
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
