/*
```cargo
[dependencies]
embassy-executor = { version = "0.7", features = ["executor-thread", "arch-riscv32"] }
embedded-test = { path = "../../..", features = ["embassy"] }

[lib]
harness = false
```
 */

#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod tests1 {

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

#[cfg(test)]
#[embedded_test::tests]
mod tests2 {

    struct Context;

    #[init]
    async fn sync_init() -> Context {
        Context
    }

    #[test]
    async fn takes_state2(_state: Context) {
        assert!(true)
    }

    #[test]
    async fn takes_no_state2() {
        assert!(true)
    }
}
