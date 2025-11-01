/*
```cargo
[dependencies]
embassy-executor = { version = "0.9", features = ["arch-riscv32"] }
esp-rtos = { version = "0.2.0", features = ["embassy", "esp32c6", "log-04"] }
esp-hal = { version = "1.0.0", features = ["esp32c6", "unstable"] }
embedded-test = { path = "../../..", features = ["embassy", "external-executor"] }

[lib]
harness = false
```
 */

#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests(executor=esp_rtos::embassy::Executor::new())]
mod tests {
    //use esp_hal::*;

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
