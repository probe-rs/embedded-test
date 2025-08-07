# Embedded Test Linker Script

Linker Script for the [embedded-test](https://crates.io/crates/embedded-test) crate.

Allows you to write

```toml
[dependencies]
embedded-test-linker-file = { version = "0.1.0" }

[dev-dependencies]
embedded-test = { version = "0.7.0" }
```

instead of

```toml
[dependencies]
embedded-test = { version = "0.7.0" }
```

## Why do we even need this?

embedded-test stores the testcases in a custom section of the ELF file and probe-rs reads that section to run the tests.
This section needs to be kept by the linker, so we need a custom linker script.

Cargo provides `cargo:rustc-link-arg-tests` to add a linker script for all integration tests. But there is no
corresponding instruction to add a linker script for bin/lib unit-tests (see
issue [rust-lang/10937](https://github.com/rust-lang/cargo/issues/10937)).

As a consequence we have to use `cargo:rustc-link-arg` which adds the linker script unconditionally, but requires us
to host at least the linker script in a crate added to `[dependencies]` instead of `[dev-dependencies]`.
