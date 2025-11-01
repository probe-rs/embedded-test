#![no_std]
#![no_main]

esp_bootloader_esp_idf::esp_app_desc!();

struct Context {
    #[allow(dead_code)]
    i2c0: esp_hal::peripherals::I2C0<'static>,
}

/// Sets up the logging before entering the test-body, so that embedded-test internal logs (e.g. Running Test <...>)  can also be printed.
/// Note: you can also inline this method in the attribute. e.g. `#[embedded_test::tests(setup=rtt_target::rtt_init_log!())]`
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
mod tests {
    use super::*;
    use esp_hal::clock::CpuClock;
    use esp_hal::timer::timg::TimerGroup;

    // Optional: A init function which is called before every test
    // asyncness of init fn is optional
    #[init]
    async fn init() -> Context {
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let peripherals = esp_hal::init(config);

        let timg0 = TimerGroup::new(peripherals.TIMG0);
        let sw_interrupt =
            esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
        esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

        // The init function can return some state, which can be consumed by the testcases
        Context {
            i2c0: peripherals.I2C0,
        }
    }

    // A test which takes the state returned by the init function (optional)
    // asyncness of test fn's is optional
    #[test]
    async fn takes_state(_state: Context) {
        assert!(true)
    }

    // Example for a test which is conditionally enabled
    #[test]
    #[cfg(feature = "log")]
    fn log() {
        log::info!("Hello, log!");
        assert!(true)
    }

    // Another example for a conditionally enabled test
    #[test]
    #[cfg(feature = "defmt")]
    fn defmt() {
        defmt::info!("Hello, defmt!");
        assert!(true)
    }

    // A test which is cfg'ed out
    #[test]
    #[cfg(abc)]
    fn it_works_disabled() {
        assert!(false)
    }

    // Tests can be ignored with the #[ignore] attribute
    #[test]
    #[ignore]
    fn it_works_ignored() {
        assert!(false)
    }

    // A test that fails with a panic
    #[test]
    fn it_fails1() {
        assert!(false)
    }

    // A test that fails with a returned Err(&str)
    #[test]
    fn it_fails2() -> Result<(), &'static str> {
        Err("It failed because ...")
    }

    // Tests can be annotated with #[should_panic] if they are expected to panic
    #[test]
    #[should_panic]
    fn it_passes() {
        assert!(false)
    }

    // This test should panic, but doesn't => it fails
    #[test]
    #[should_panic]
    fn it_fails3() {}

    // Tests can be annotated with #[timeout(<secs>)] to change the default timeout of 60s
    #[test]
    #[timeout(10)]
    fn it_timeouts() {
        loop {} // should run into the 10s timeout
    }
}
