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
    fn helper() -> u32 {
        42
    }

    #[test]
    fn takes() {
        assert_eq!(helper(), 42)
    }
}
