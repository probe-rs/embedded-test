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
    #[test]
    async fn takes_no_state() {
        assert!(true)
    }
}
