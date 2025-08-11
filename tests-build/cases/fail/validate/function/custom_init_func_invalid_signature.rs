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

    fn custom_init(_a: u32) -> Context {
        Context
    }

    #[test(init = custom_init)]
    fn takes_state(_state: Context) {
        assert!(true)
    }
}
