// Copied from https://github.com/knurling-rs/defmt/blob/main/firmware/defmt-test/src/lib.rs

#![no_std]

mod fmt;

pub use embedded_test_macros::tests;

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
