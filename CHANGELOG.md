# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.6.1]

### Added

- Support [Ariel OS](https://ariel-os.org).
- Support to run test on the host/std for Ariel OS.

### Changed
- Updated defmt 0.3.8 => 1


## [0.6.0]

### Added

- `#[tests(default_timeout = <u32>)]` to configure a suite-wide default timeout.
- `#[tests(setup = <expr>)]` to configure a suite-wide (log) setup function (e.g. `rtt_target::rtt_init_log()`).

### Removed

- Breaking: Removed Features `init-log` and `init-rtt`.

### Changed

- Breaking: Bump embassy-excecutor to 0.7.0

## [0.5.0]

### Changed

- Breaking: Bump embassy-excecutor to 0.6.1

## [0.4.0]

### Added

- Make it possible to bring your own Embassy executor (feature `external-executor`)
- Added panic handler directly to this crate (enabled per default, feature `panic-handler`)
- Added support for xtensa semihosting (feature `xtensa-semihosting`)
- Added feature to initialize logging sink (feature `init-log`)
- Breaking: Added a linker script, to ensure symbols like `EMBEDDED_TEST_VERSION` are kept

### Changed

- Feature `rtt` renamed to `init-rtt` to better reflect its purpose.

## [0.3.0]

### Added

- Added Feature `rtt` to initialize logging via `rtt-target` crate.

### Changed

- Breaking: Bump embassy-excecutor to 0.5.0

## [0.2.3]

### Added

- Show improved diagnostic when no executor feature is enabled on the embassy-executor crate.
- Calculate the test list buffer size at compile time to avoid a too small buffer.

### Fixed

- Macro produced invalid rust code when there was no #[init] function present.

## [0.2.2]

### Changed

- Removed `#![feature(trait_alias)]` to allow usage of `embedded-test` in stable rust.

### Fixed

- Updated `semihosting` dependency to fix failing build for cortex-m targets.

## [0.2.1]

Initial release on crates.io

[unreleased]: https://github.com/probe-rs/embedded-test/compare/v0.6.1...master

[0.6.1]: https://github.com/probe-rs/embedded-test/compare/v0.6.0...v0.6.1

[0.6.0]: https://github.com/probe-rs/embedded-test/compare/v0.5.0...v0.6.0

[0.5.0]: https://github.com/probe-rs/embedded-test/compare/v0.4.0...v0.5.0

[0.4.0]: https://github.com/probe-rs/embedded-test/compare/v0.3.0...v0.4.0

[0.3.0]: https://github.com/probe-rs/embedded-test/compare/v0.2.3...v0.3.0

[0.2.3]: https://github.com/probe-rs/embedded-test/compare/v0.2.2...v0.2.3

[0.2.2]: https://github.com/probe-rs/embedded-test/compare/v0.2.1...v0.2.2

[0.2.1]: https://github.com/probe-rs/embedded-test/releases/tag/v0.2.1
