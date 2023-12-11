#![allow(dead_code)]

//Copied from https://github.com/taiki-e/semihosting

use core::{
    arch::asm,
    ffi::{c_int, c_void, CStr},
    marker::PhantomData,
};

/// PARAMETER REGISTER (read-write)
#[repr(transparent)]
pub(crate) struct ParamRegW<'a>(pub(crate) *mut c_void, PhantomData<&'a mut ()>);
impl<'a> ParamRegW<'a> {
    /*
    #[inline]
    pub(crate) fn fd(fd: BorrowedFd<'a>) -> Self {
        Self::raw_fd(fd.as_raw_fd())
    }
    #[inline]
    pub(crate) fn raw_fd(fd: RawFd) -> Self {
        Self::isize(fd as isize)
    }*/
    #[inline]
    pub(crate) fn usize(n: usize) -> Self {
        Self(n as *mut c_void, PhantomData)
    }
    #[allow(clippy::cast_sign_loss)]
    #[inline]
    pub(crate) fn isize(n: isize) -> Self {
        Self::usize(n as usize)
    }
    #[inline]
    pub(crate) fn ptr<T>(ptr: *mut T) -> Self {
        Self(ptr.cast::<c_void>(), PhantomData)
    }
    #[inline]
    pub(crate) fn ref_<T>(r: &'a mut T) -> Self {
        Self::ptr(r)
    }
    #[inline]
    pub(crate) fn buf<T>(buf: &'a mut [T]) -> Self {
        Self::ptr(buf.as_mut_ptr())
    }
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "riscv32",
    target_arch = "riscv64",
))]
impl<'a> ParamRegW<'a> {
    #[inline]
    pub(crate) fn block(b: &'a mut [ParamRegW<'_>]) -> Self {
        Self::ptr(b.as_mut_ptr())
    }
}

/// PARAMETER REGISTER (read-only)
#[repr(transparent)]
pub(crate) struct ParamRegR<'a>(pub(crate) *const c_void, PhantomData<&'a ()>);
impl<'a> ParamRegR<'a> {
    /*
    #[inline]
    pub(crate) fn fd(fd: BorrowedFd<'a>) -> Self {
        Self::raw_fd(fd.as_raw_fd())
    }
    #[inline]
    pub(crate) fn raw_fd(fd: RawFd) -> Self {
        Self::isize(fd as isize)
    }*/
    #[inline]
    pub(crate) fn usize(n: usize) -> Self {
        Self(n as *const c_void, PhantomData)
    }
    #[allow(clippy::cast_sign_loss)]
    #[inline]
    pub(crate) fn isize(n: isize) -> Self {
        Self::usize(n as usize)
    }
    #[inline]
    pub(crate) fn ptr<T>(ptr: *const T) -> Self {
        Self(ptr.cast::<c_void>(), PhantomData)
    }
    #[inline]
    pub(crate) fn buf<T>(buf: &'a [T]) -> Self {
        Self::ptr(buf.as_ptr())
    }
    #[inline]
    pub(crate) fn c_str(s: &'a CStr) -> Self {
        Self::ptr(s.as_ptr())
    }
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "riscv32",
    target_arch = "riscv64",
))]
impl<'a> ParamRegR<'a> {
    #[inline]
    pub(crate) fn block(b: &'a [ParamRegR<'_>]) -> Self {
        Self::ptr(b.as_ptr())
    }
    #[inline]
    pub(crate) fn ref_<T>(r: &'a T) -> Self {
        Self::ptr(r)
    }
}

/// RETURN REGISTER
#[derive(Clone, Copy)]
#[repr(transparent)]
pub(crate) struct RetReg(pub(crate) *mut c_void);
impl RetReg {
    #[inline]
    pub(crate) fn usize(self) -> usize {
        self.0 as usize
    }
    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    #[inline]
    fn isize(self) -> isize {
        self.usize() as isize
    }
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    pub(crate) fn int(self) -> c_int {
        self.isize() as c_int
    }
    /*
    #[inline]
    pub(crate) fn raw_fd(self) -> Option<RawFd> {
        let fd = self.int();
        if fd == -1 {
            None
        } else {
            debug_assert!(!fd.is_negative(), "{}", fd);
            debug_assert_eq!(fd as isize, self.isize());
            Some(fd)
        }
    }
    #[inline]
    pub(crate) fn errno(self) -> RawOsError {
        let err = self.int();
        debug_assert!(!err.is_negative(), "{}", err);
        debug_assert_eq!(err as isize, self.isize());
        err
    }*/
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "riscv32",
    target_arch = "riscv64",
))]
impl RetReg {
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    pub(crate) fn u8(self) -> u8 {
        let b = self.usize() as u8;
        debug_assert_eq!(b as usize, self.usize());
        b
    }
}

#[cfg(target_arch = "riscv32")]
pub(crate) unsafe fn syscall(number: usize, parameter: ParamRegW<'_>) -> RetReg {
    unsafe {
        let r;
        asm!(
        ".balign 16",
        ".option push",
        ".option norvc",
        "slli zero, zero, 0x1F",
        "ebreak",
        "srai zero, zero, 0x7",
        ".option pop",
        inout("a0") number as usize => r, // OPERATION NUMBER REGISTER => RETURN REGISTER
        // Use inout because operation such as SYS_ELAPSED suggest that
        // PARAMETER REGISTER may be changed.
        inout("a1") parameter.0 => _, // PARAMETER REGISTER
        options(nostack, preserves_flags),
        );
        RetReg(r)
    }
}

#[cfg(target_arch = "riscv32")]
pub(crate) unsafe fn syscall_readonly(number: usize, parameter: ParamRegR<'_>) -> RetReg {
    unsafe {
        let r;
        asm!(
        ".balign 16",
        ".option push",
        ".option norvc",
        "slli zero, zero, 0x1F",
        "ebreak",
        "srai zero, zero, 0x7",
        ".option pop",
        inout("a0") number as usize => r, // OPERATION NUMBER REGISTER => RETURN REGISTER
        // Use inout because operation such as SYS_ELAPSED suggest that
        // PARAMETER REGISTER may be changed.
        inout("a1") parameter.0 => _, // PARAMETER REGISTER
        options(nostack, preserves_flags, readonly),
        );
        RetReg(r)
    }
}
