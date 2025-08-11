/*
```cargo
[dependencies]
embedded-test = { path = "../../../.." }
[lib]
harness = false
```
 */

#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::setup]
fn setup() {}

mod somewhere_else {
    #[cfg(test)]
    #[embedded_test::setup]
    fn setup() {}
}

#[cfg(test)]
#[embedded_test::tests]
mod tests {

    #[test]
    fn ok() {
        assert!(true);
    }
}
