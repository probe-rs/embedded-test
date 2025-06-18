#[cfg(test)]
#[embedded_test::tests(executor=esp_hal_embassy::Executor::new())]
mod tests {
    use esp_hal::prelude::*;
    #[test]
    async fn banana_1() {
        assert!(true);
    }

    #[test]
    #[should_panic]
    async fn banana_2() {
        assert!(false);
    }
}
