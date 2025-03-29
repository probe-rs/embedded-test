use semihosting::experimental::env::Args;
use semihosting::io;
use semihosting::sys::arm_compat::syscall::ParamRegR;
use semihosting::sys::arm_compat::syscall::{syscall_readonly, OperationNumber};

use crate::export::Tests;

pub fn args() -> io::Result<Args<1024>> {
    semihosting::experimental::env::args::<1024>()
}

pub fn abort() -> ! {
    semihosting::process::abort()
}

pub fn exit(code: i32) -> ! {
    semihosting::process::exit(code)
}

pub fn print_test_list<const JSON_SIZE_TOTAL: usize>(tests: &Tests<'_>) {
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
        abort();
    }
}
