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
    #[init]
    fn init() -> u32 {
        42
    }

    fn custom_init() {}

    #[test(init = custom_init)]
    fn test(_state: u32) {}
}
