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
mod tests;
