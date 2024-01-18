# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) with Cargo behavior for pre-1.0.0 versions.

## [1.0.0] - 2024-01-18

## Changed

- Removed needs for nightly toolchain, MSRV is 1.75 for the async crate and examples
- Updated `embedded-hal` and `embedded-hal-async` to 1.0.0
- Updated dependencies for examples
- Updated READMEs

## [0.2.0] - 2023-09-28

## Changed

- Updated `embedded-hal` and `embedded-hal-async`
- Updated dependencies for examples

## [0.1.0] - 2023-05-16

### Added

- ADXL355 blocking driver.
- ADXL355 async driver.
- Examples using a BMD-340-EVAL which features the Nordic nRF52840 RF System on Chip.
- Example using a Raspberry Pi 4 running the Raspberry Pi OS.

[0.1.0]: https://github.com/analogdevicesinc/adi-adxl355-rs/tree/0.1.0
[0.2.0]: https://github.com/analogdevicesinc/adi-adxl355-rs/compare/0.1.0...0.2.0
[1.0.0]: https://github.com/analogdevicesinc/adi-adxl355-rs/compare/0.2.0...1.0.0
