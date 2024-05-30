# Embedded Test

[![Crates.io](https://img.shields.io/crates/v/embedded-test?labelColor=1C2C2E&color=C96329&logo=Rust&style=flat-square)](https://crates.io/crates/embedded-test)
![Crates.io](https://img.shields.io/crates/l/embedded-test?labelColor=1C2C2E&style=flat-square)

The embedded-test library provides a test harness for embedded systems (riscv, arm and xtensa).
Use this library on the target together with [probe-rs](https://probe.rs/) on the host to run integration tests on your
embedded device.

[probe-rs](https://probe.rs/)  together with embedded-test provide a (libtest compatible) test runner, which will:

1. Flash all the tests to the device in one go (via the `probe-rs run` command)
2. Request information about all tests from the device (via semihosting SYS_GET_CMDLINE)
3. In turn for each testcase:
    - Reset the device
    - Signal to the device (via semihosting SYS_GET_CMDLINE) which test to run
    - Wait for the device to signal that the test completed successfully or with error (via semihosting SYS_EXIT)
4. Report the results

Since the test runner (`probe-rs run`) is libtest compatible (
using [libtest-mimic](https://crates.io/crates/libtest-mimic)), you can use intellij or vscode to run individual tests
with the click of a button.

![](./demo.gif)

## Features

* Runs each test case individually, and resets the device between each test case
* Supports an init function which will be called before each test case and can pass state to the test cases
* Supports async test and init functions (needs feature `embassy`)
* Support `#[should_panic]`, `#[ignore]` and `#[timeout(<seconds>)]` attributes for each test case

## Usage

Add the following to your `Cargo.toml`:

```toml
[dev-dependencies]
embedded-test = { version = "0.4.0", features = ["init-log"] }


[[test]]
name = "example_test"
harness = false
```

Install the runner on your system:

```bash 
cargo install probe-rs-tools
```

Add the following to your `.cargo/config.toml`:

```toml
[target.riscv32imac-unknown-none-elf]
runner = "probe-rs run --chip esp32c6"
# `probe-rs run` will autodetect whether the elf to flash is a normal firmware or a test binary
```

Add the following to your `build.rs` file:

```rust
fn main() {
    println!("cargo::rustc-link-arg-tests=-Tembedded-test.x");
}
```

Then you can run your tests with `cargo test --test example_test` or use the button in vscode/intellij.

Having trouble setting up? Checkout out
the [FAQ and common Errors](https://github.com/probe-rs/embedded-test/wiki/FAQ-and-common-Errors) Wiki page.

## Example Test

[Example repo](https://github.com/probe-rs/embedded-test-example)  
[More Detailed Cargo.toml](https://github.com/probe-rs/embedded-test-example/blob/master/Cargo.toml)  
[Async Test Example](https://github.com/probe-rs/embedded-test-example/blob/master/tests/async_test.rs)

Example for `tests/example_test.rs`

```rust 
#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use esp_hal::{clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*};

    // Optional: A init function which is called before every test
    #[init]
    fn init() -> Delay {
        let peripherals = Peripherals::take();
        let system = peripherals.SYSTEM.split();
        let clocks = ClockControl::max(system.clock_control).freeze();
        let delay = Delay::new(&clocks);

        // The init function can return some state, which can be consumed by the testcases
        delay
    }

    // A test which takes the state returned by the init function (optional)
    #[test]
    fn takes_state(_state: Delay) {
        assert!(true)
    }

    // Example for a test which is conditionally enabled
    #[test]
    #[cfg(feature = "log")]
    fn log() {
        log::info!("Hello, log!"); // Prints via esp-println to rtt
        assert!(true)
    }

    // Another example for a conditionally enabled test
    #[test]
    #[cfg(feature = "defmt")]
    fn defmt() {
        use defmt_rtt as _;
        defmt::info!("Hello, defmt!"); // Prints via defmt-rtt to rtt
        assert!(true)
    }

    // A test which is cfg'ed out
    #[test]
    #[cfg(abc)]
    fn it_works_disabled() {
        assert!(false)
    }

    // Tests can be ignored with the #[ignore] attribute
    #[test]
    #[ignore]
    fn it_works_ignored() {
        assert!(false)
    }

    // A test that fails with a panic
    #[test]
    fn it_fails1() {
        assert!(false)
    }

    // A test that fails with a returned Err(&str)
    #[test]
    fn it_fails2() -> Result<(), &'static str> {
        Err("It failed because ...")
    }

    // Tests can be annotated with #[should_panic] if they are expected to panic
    #[test]
    #[should_panic]
    fn it_passes() {
        assert!(false)
    }

    // This test should panic, but doesn't => it fails
    #[test]
    #[should_panic]
    fn it_fails3() {}

    // Tests can be annotated with #[timeout(<secs>)] to change the default timeout of 60s
    #[test]
    #[timeout(10)]
    fn it_timeouts() {
        loop {} // should run into the 10s timeout
    }
}
```

## Configuration features

| Feature              | Default? | Description                                                                                                                                                                                   |
|----------------------|----------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `panic-handler`      | Yes      | Defines a panic-handler which will invoke `semihosting::process::abort()` on panic                                                                                                            |
| `defmt`              | No       | Prints testcase exit result to defmt. You'll need to setup your defmt `#[global_logger]` yourself.                                                                                            |
| `log`                | No       | Prints testcase exit result to log. You'll need to setup your logging sink yourself (or combine with the `init-log` feature).                                                                 |
| `init-rtt`           | No       | Calls `rtt_target::rtt_init_print!()` before starting any tests                                                                                                                               |
| `init-log`           | No       | Calls `rtt_log::init();` before starting any tests.                                                                                                                                           |
| `embassy`            | No       | Enables async test and init functions. Note: You need to enable at least one executor feature on the embassy-executor crate unless you are using the `external-executor` feature.             |
| `external-executor`  | No       | Allows you to bring your own embassy executor which you need to pass to the `#[tests]` macro (e.g. `#[embedded_test::tests(executor = esp_hal::embassy::executor::thread::Executor::new())]`) |
| `xtensa-semihosting` | No       | Enables semihosting for xtensa targets.                                                                                                                                                       |

## License

Licensed under either of:

- Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.

