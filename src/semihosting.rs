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
