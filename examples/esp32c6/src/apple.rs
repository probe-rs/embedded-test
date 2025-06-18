#[cfg(test)]
#[embedded_test::tests(executor=esp_hal_embassy::Executor::new(), default_timeout=100)]
mod tests {
    use esp_hal::prelude::*;

    #[init]
    fn init() -> bool {
        true
    }

    #[test]
    fn apple_1(state: bool) {
        assert!(true);
    }

    #[test]
    #[should_panic]
    #[timeout(23)]
    async fn apple_2(state: bool) {
        assert!(false);
    }
}
