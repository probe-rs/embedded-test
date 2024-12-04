#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests(setup = rtt_target::rtt_init_log!(), executor=esp_hal_embassy::Executor::new() )]
mod test {
    use embassy_time::{Duration, Timer};
    use esp_hal::prelude::*;

    #[init]
    fn init() -> () {
        let peripherals = esp_hal::init({
            let mut config = esp_hal::Config::default();
            config.cpu_clock = CpuClock::max();
            config
        });

        let timer0 = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER)
            .split::<esp_hal::timer::systimer::Target>();
        esp_hal_embassy::init(timer0.alarm0);
    }

    #[test]
    fn test() {
        log::error!("Hello, log!");
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn it_fails1() {
        assert!(false)
    }

    #[test]
    fn it_fails2() -> Result<(), &'static str> {
        Err("It failed because ...")
    }

    #[test]
    #[should_panic]
    fn it_passes() {
        assert!(false)
    }

    // This test should panic, but doesn't => it fails
    #[test]
    #[should_panic]
    fn it_fails3() {}

    #[test]
    #[timeout(3)]
    fn it_timeouts() {
        loop {} // should run into the 3s timeout
    }

    #[test]
    async fn async_test() {
        Timer::after(Duration::from_millis(100)).await;
        assert_eq!(1 + 1, 2);
    }
}
