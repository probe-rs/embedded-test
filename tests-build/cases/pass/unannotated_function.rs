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

    fn helper() -> u32 {
        42
    }

    #[test]
    fn takes_no_state() {
        assert_eq!(helper(), 42);
    }
}
