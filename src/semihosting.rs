use semihosting::experimental::env::Args;
use semihosting::io;

pub fn args() -> io::Result<Args<1024>> {
    semihosting::experimental::env::args::<1024>()
}

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
