# Embedded Test

The embedded-test library provides a test hardness for embedded systems (riscv and arm). It is based on the idea of [defmt-test](https://crates.io/crates/defmt-test).

`probe-rs test` provides a (libtest compatible) test runner, which will:
1. Flash all the tests to the device in one go (via probe-rs)
2. Request information about all tests from the device (via semihosting SYS_GET_CMDLINE)
3. In turn for each testcase: 
   - Reset the device
   - Signal to the device (via semihosting SYS_GET_CMDLINE) which test to run
   - Wait for the device to signal that the test completed successfully or with error (via semihosting SYS_EXIT)
4. Report the results

Since the test runner (`probe-rs test`) is libtest compatible (using [libtest-mimic](https://crates.io/crates/libtest-mimic)), you can use intellij or vscode to run individual tests with the click of a button.

![](./demo.gif)


## WARNING
This project is development state. Don't rely on it for anything important yet.

## Usage

Add the following to your `Cargo.toml`:

```toml
[[test]]
name = "example_test"
harness = false

[dev-dependencies]
embedded-test = {version="0.2.0", features = ["log"]} # enable log or defmt to see some debug output
panic-probe = {git = "https://github.com/t-moe/defmt"}  # the upstream create does not support riscv yet
```

Install the runner on your system:
```bash 
cargo install probe-rs --git https://github.com/probe-rs/probe-rs --branch feature/testing --features cli --bin probe-rs
```

Add the following to your `.cargo/config.toml`:

```toml
[target.riscv32imac-unknown-none-elf]

# Syntax is: probe-rs test <flash settings> -- <elf> <libtest args>
runner = "probe-rs test --chip esp32c6 -- "
```

Then you can run your tests with `cargo test` or use the button in vscode/intellij.

## Example Test

[Example repo](https://github.com/probe-rs/embedded-test-example)

Example for `tests/example_test.rs`

```rust 
#![no_std]
#![no_main]

use esp32c6_hal as _; // exception handler
use panic_probe as _; // semihosting::process::abort on test failure

#[cfg(test)]
#[embedded_test::tests]
mod unit_tests {

    #[test]
    fn it_works() {
        assert!(true)
    }

    #[test]
    #[cfg(abc)]
    fn it_works2() {
        assert!(false)
    }

    #[test]
    #[ignore]
    #[cfg(not(abc))]
    fn it_works3() {
        assert!(false)
    }

    #[test]
    #[cfg(not(abc))]
    fn it_works4() {
        assert!(false)
    }
}
```

