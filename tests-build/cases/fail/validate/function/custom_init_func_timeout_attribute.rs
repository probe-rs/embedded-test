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

    #[timeout(13)]
    fn custom_init() -> Context {
        Context
    }

    #[test(init = custom_init)]
    fn takes_state(_state: Context) {
        assert!(true)
    }
}
