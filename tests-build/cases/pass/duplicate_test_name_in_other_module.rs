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
mod tests1 {
    #[test]
    fn takes_no_state() {
        assert!(true)
    }
}

#[cfg(test)]
#[embedded_test::tests]
mod tests2 {
    #[test]
    fn takes_no_state() {
        assert!(true)
    }
}
