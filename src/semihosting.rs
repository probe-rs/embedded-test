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
    // TODO: print reason
    abort();
}

pub fn print_test_list() {
    // TODO: print reason: We no longer print the tests via semihosting, they should be read from the ELF with probe-rs
    abort();
}
