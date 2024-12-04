#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests(setup=rtt_target::rtt_init_log!())]
mod test {
    use embassy_time::{Duration, Timer};

    #[init]
    fn init() -> () {
        let _p = embassy_nrf::init(Default::default());
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
