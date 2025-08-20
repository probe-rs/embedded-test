/*
```cargo
[dependencies]
embassy-executor = { version = "0.8", features = ["arch-riscv32"] }
embedded-test = { path = "../../../../..", features = ["embassy", "external-executor"] }

[lib]
harness = false
```
 */

#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    struct Context;

    async fn custom_init() -> Context {
        Context
    }

    #[test(init=custom_init)]
    fn takes_state(_state: Context) {
        assert!(true)
    }
}
