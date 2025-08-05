#![no_main]

#[cfg(test)]
#[cfg(feature = "log")]
#[embedded_test::setup]
fn setup() {
    env_logger::init();
}

#[cfg(test)]
#[embedded_test::tests]
mod unit_tests {
    // Optional: A init function which is called before every test
    // async + return value are optional
    #[init]
    async fn init() -> u32 {
        1234
    }

    // A test which takes the state returned by the init function (optional)
    // asyncness is optional and needs feature embassy
    #[test]
    async fn takes_state(_state: u32) {
        assert!(true)
    }

    // Example for a test which is conditionally enabled
    #[test]
    #[cfg(feature = "log")]
    fn log() {
        log::error!("Hello, log!");
        assert!(false)
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
    #[timeout(2)]
    fn it_timeouts() {
        loop {} // should run into the 10s timeout
    }
}
