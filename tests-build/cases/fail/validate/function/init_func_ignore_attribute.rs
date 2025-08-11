/*
```cargo
[dependencies]
embedded-test = { path = "../../../../.." }
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

    #[ignore]
    #[init]
    fn init() -> Context {
        Context
    }

    #[test]
    fn takes_state(_state: Context) {
        assert!(true)
    }
}
