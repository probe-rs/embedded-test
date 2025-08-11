/*
```cargo
[dependencies]
embassy-executor = { version = "0.7", features = ["executor-thread","arch-riscv32"] }
esp-hal-embassy = { version = "0.6", features = ["esp32c6"] }
embedded-test = { path = "../../../../..", features = ["embassy"] }

[lib]
harness = false
```
 */

#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests(executor=esp_hal_embassy::Executor::new())]
mod tests {

    struct Context;

    #[init]
    async fn async_init() -> Context {
        Context
    }

    #[test]
    async fn takes_state(_state: Context) {
        assert!(true)
    }

    #[test]
    async fn takes_no_state() {
        assert!(true)
    }
}
