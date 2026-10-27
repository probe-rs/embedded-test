use semihosting::experimental::env::Args;
use semihosting::io;

pub fn args() -> io::Result<Args<1024>> {
    semihosting::experimental::env::args::<1024>()
}

pub fn setup() {
    // The embedded-test.x linker script points this at the user provided setup function, or at the
    // default below if the user did not provide one.
    extern "Rust" {
        fn _embedded_test_setup();
    }
    unsafe { _embedded_test_setup() }
}

#[export_name = "__embedded_test_default_setup"]
fn default_setup() {}

pub fn abort() -> ! {
    semihosting::process::abort()
}

pub fn exit(code: i32) -> ! {
    semihosting::process::exit(code)
}

pub fn run_test(_test: &str) -> ! {
    error!("Running test by name is no longer supported by embedded-test. Please upgrade probe-rs to the latest version");
    abort();
}

pub fn print_test_list() -> ! {
    error!("Querying tests via semihosting is no longer supported by embedded-test. Please upgrade probe-rs to the latest version");
    abort();
}
