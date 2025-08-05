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
