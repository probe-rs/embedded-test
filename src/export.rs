
use crate::TestOutcome;

#[cfg(all(feature = "defmt", feature = "log"))]
compile_error!("You may not enable both `defmt` and `log` features.");

#[cfg(all(not(feature = "defmt"), not(feature = "log")))]
compile_error!("You have to enable either the `defmt` or the `log` feature");


/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
pub fn exit() -> ! {
    loop {
        semihosting::process::exit(0);
    }
}

#[cfg(feature = "defmt")]
pub fn check_outcome<T: TestOutcome>(outcome: T, should_error: bool) {
    if outcome.is_success() == should_error {
        let note = if should_error {
            defmt::intern!("`#[should_error]` ")
        } else {
            defmt::intern!("")
        };
        defmt::panic!("{}test failed with outcome: {}", note, outcome);
    }
}

#[cfg(feature = "log")]
pub fn check_outcome<T: TestOutcome>(outcome: T, should_error: bool) {
    if outcome.is_success() == should_error {
        let note = if should_error {
            "`#[should_error]` "
        } else {
            ""
        };
        core::panic!("{}test failed with outcome: {:?}", note, outcome);
    }
}