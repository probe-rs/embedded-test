#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::setup]
fn setup() {
    #[cfg(feature = "log")]
    rtt_target::rtt_init_log!();
    #[cfg(feature = "defmt")]
    rtt_target::rtt_init_defmt!();
}

#[cfg(test)]
#[embedded_test::tests]
mod unit_tests {
    use stm32f7xx_hal as _; // to bring exception handlers into scope
    #[test]
    #[cfg(feature = "log")]
    fn test_one() {
        log::error!("Very bad error message! You may only see this if you make the test fail");
        assert!(true);
    }

    #[test]
    async fn test_two() {
        assert!(true);
    }
}
