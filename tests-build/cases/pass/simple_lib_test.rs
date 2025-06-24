/*
```cargo
[dependencies]
embedded-test = { path = "../../.." }
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
    fn init() -> Context {
        Context
    }

    #[test]
    fn takes_state(_state: Context) {
        assert!(true)
    }

    #[test]
    fn takes_no_state() {
        assert!(true)
    }

    #[test]
    #[cfg(abc)]
    fn cfged_out() {
        assert!(false)
    }

    #[test]
    #[ignore]
    fn ignored() {
        assert!(false)
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        assert!(false)
    }

    #[test]
    #[timeout(10)]
    fn custom_timeout() {
        loop {}
    }
}
