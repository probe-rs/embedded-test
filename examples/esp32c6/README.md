# Embedded Test Example for the esp32c6 (riscv32)

## Platform-specific notes:

- When using async-tests:
    - esp-rs provides an optimized embassy-executor, therefore the feature `custom-executor` must be enabled on
      embedded-test and the invocation of embedded test should be
      `#[embedded_test::tests(executor=esp_rtos::embassy::Executor::new())]`
