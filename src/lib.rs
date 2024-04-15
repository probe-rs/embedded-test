// Copied from https://github.com/knurling-rs/defmt/blob/main/firmware/defmt-test/src/lib.rs

#![no_std]

mod fmt;

pub use embedded_test_macros::tests;

#[cfg(feature = "panic-handler")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(not(nightly))]
    {
        error!("!! A panic occured: {:#?}", info);
    }

    #[cfg(nightly)]
    {
        if let Some(location) = info.location() {
            let (file, line, column) = (location.file(), location.line(), location.column());
            error!(
                "!! A panic occured in '{}', at line {}, column {}:",
                file, line, column
            );
        } else {
            error!("!! A panic occured at an unknown location:");
        }
        if let Some(message) = info.message() {
            error!("{}", message);
        }
    }

    semihosting::process::abort()
}

/// Private implementation details used by the proc macro.
/// WARNING: This API is not stable and may change at any time.
#[doc(hidden)]
pub mod export;

mod sealed {
    pub trait Sealed {}
    impl Sealed for () {}
    impl<T, E> Sealed for Result<T, E> {}
}

/// Indicates whether a test succeeded or failed.
///
/// This is comparable to the `Termination` trait in libstd, except stable and tailored towards the
/// needs of embedded-test. It is implemented for `()`, which always indicates success, and `Result`,
/// where `Ok` indicates success.
#[cfg(feature = "defmt")]
pub trait TestOutcome: defmt::Format + sealed::Sealed {
    fn is_success(&self) -> bool;
}

/// Indicates whether a test succeeded or failed.
///
/// This is comparable to the `Termination` trait in libstd, except stable and tailored towards the
/// needs of embedded-test. It is implemented for `()`, which always indicates success, and `Result`,
/// where `Ok` indicates success.
#[cfg(feature = "log")]
pub trait TestOutcome: core::fmt::Debug + sealed::Sealed {
    fn is_success(&self) -> bool;
}

/// Indicates whether a test succeeded or failed.
///
/// This is comparable to the `Termination` trait in libstd, except stable and tailored towards the
/// needs of embedded-test. It is implemented for `()`, which always indicates success, and `Result`,
/// where `Ok` indicates success.
#[cfg(all(not(feature = "log"), not(feature = "defmt")))]
pub trait TestOutcome: sealed::Sealed {
    fn is_success(&self) -> bool;
}

impl TestOutcome for () {
    fn is_success(&self) -> bool {
        true
    }
}

#[cfg(feature = "log")]
impl<T: core::fmt::Debug, E: core::fmt::Debug> TestOutcome for Result<T, E> {
    fn is_success(&self) -> bool {
        self.is_ok()
    }
}

#[cfg(feature = "defmt")]
impl<T: defmt::Format, E: defmt::Format> TestOutcome for Result<T, E> {
    fn is_success(&self) -> bool {
        self.is_ok()
    }
}

#[cfg(all(not(feature = "log"), not(feature = "defmt")))]
impl<T, E> TestOutcome for Result<T, E> {
    fn is_success(&self) -> bool {
        self.is_ok()
    }
}
