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
#[embedded_test::tests(default_timeout = 150)]
mod tests {
    #[test]
    fn takes_no_state() {
        assert!(true)
    }

    #[test]
    #[timeout(10)]
    fn custom_timeout() {
        loop {}
    }
}
