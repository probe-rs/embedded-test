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
    fn init1() {}

    #[test(init = my_custom_init)]
    fn test(_state: u32) {}
}
