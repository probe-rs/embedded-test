#![no_std]
#![no_main]

esp_bootloader_esp_idf::esp_app_desc!();

#[cfg(test)]
#[embedded_test::setup]
fn setup_log() {
    #[cfg(feature = "log")]
    rtt_target::rtt_init_log!();
    #[cfg(feature = "defmt")]
    rtt_target::rtt_init_defmt!();
}

#[cfg(test)]
#[embedded_test::tests(executor=esp_rtos::embassy::Executor::new())]
mod unit_tests {
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
