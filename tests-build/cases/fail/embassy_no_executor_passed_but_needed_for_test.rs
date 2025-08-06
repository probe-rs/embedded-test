/*
```cargo
[dependencies]
embassy-executor = { version = "0.7", features = ["arch-riscv32"] }
embedded-test = { path = "../../..", features = ["embassy", "external-executor"] }

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

    #[init]
    fn sync_init() -> Context {
        Context
    }

    #[test]
    async fn takes_state(_state: Context) {
        assert!(true)
    }
}
