use crate::{export, TestOutcome};

#[cfg_attr(feature = "std", path = "std.rs")]
#[cfg_attr(feature = "semihosting", path = "semihosting.rs")]
pub mod hosting;

// Reexport the embassy stuff
#[cfg(all(feature = "embassy", not(feature = "ariel-os")))]
pub use embassy_executor::task;
#[cfg(all(
    feature = "embassy",
    not(feature = "external-executor"),
    not(feature = "ariel-os")
))]
pub use embassy_executor::Executor; // Please activate the `executor-thread` or `executor-interrupt` feature on the embassy-executor crate (v0.7.x)!

pub fn check_outcome<T: TestOutcome>(outcome: T) -> ! {
    if outcome.is_success() {
        info!("Test exited with () or Ok(..)");
        hosting::exit(0);
    } else {
        info!("Test exited with Err(..): {:?}", outcome);
        hosting::abort();
    }
}

/*
// TODO: Readd support for ariel os startup. Ariel OS is currently not published to crates.io
#[cfg(feature = "ariel-os")]
#[ariel_os::thread(autostart, stacksize = 16384)]
fn embedded_test_thread() {
    // TODO: make stack size configurable
    unsafe { __embedded_test_entry() }
}*/

#[cfg_attr(not(feature = "ariel-os"), export_name = "main")]
pub unsafe extern "C" fn __embedded_test_entry() -> ! {
    ensure_linker_file_was_added_to_rustflags();
}

fn ensure_linker_file_was_added_to_rustflags() -> ! {
    // Try to access a symbol which we provide in the embedded-test.x linker file.
    // The linker file will redirect this call to the function below.
    // This will trigger a linker error if the linker file has not been added to the rustflags
    extern "C" {
        fn embedded_test_linker_file_not_added_to_rustflags() -> !;
    }
    unsafe { embedded_test_linker_file_not_added_to_rustflags() }
}

#[no_mangle]
unsafe extern "C" fn __embedded_test_start() -> ! {
    // Invoke the user provided setup function, if it exists or run a default (empty) setup function.
    extern "Rust" {
        fn _embedded_test_setup();
    }
    unsafe { _embedded_test_setup() }

    let args = &export::hosting::args().expect("Failed to get cmdline via semihosting");
    // this is an iterator already with semihosting, not on std
    let mut args = args.into_iter();

    let command = match args.next() {
        Some(c) => c.expect("command to run contains non-utf8 characters"),
        None => {
            error!("Received no arguments via semihosting. Please communicate with the target with the embedded-test runner.");
            export::hosting::abort();
        }
    };

    match command {
        "list" => {
            export::hosting::print_test_list();
        }
        "run" => {
            let test_name = args.next().expect("test name missing");
            let test_name = test_name.expect("test name contains non-utf8 character");
            export::hosting::run_test(test_name);
        }
        "run_addr" => {
            let addr = args.next().expect("addr missing");
            let addr = addr.expect("addr contains non-utf8 character");
            let addr: usize = addr.parse().expect("invalid number");
            let test_invoker: fn() -> ! = unsafe { core::mem::transmute(addr) };
            test_invoker();
        }
        _ => {
            error!("Unknown command: {}", command);
            export::hosting::abort();
        }
    }
}

#[export_name = "__embedded_test_default_setup"]
fn default_setup() {}

#[used]
#[no_mangle]
#[link_section = ".embedded_test.meta"]
static EMBEDDED_TEST_VERSION: usize = 1; // Format version of our protocol between probe-rs and target running embedded-test
