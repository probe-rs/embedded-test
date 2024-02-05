# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
## [Unreleased]
### Added

### Changed

### Fixed

### Removed

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

[unreleased]: https://github.com/probe-rs/embedded-test/compare/v0.3.0...master
[0.3.0]: https://github.com/probe-rs/embedded-test/compare/v0.2.3...v0.3.0
[0.2.3]: https://github.com/probe-rs/embedded-test/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/probe-rs/embedded-test/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/probe-rs/embedded-test/releases/tag/v0.2.1
