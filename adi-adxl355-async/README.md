# ADXL355 Async Driver

Platform-agnostic Rust async driver for the ADXL355 accelerometer.

## Product info

The ADXL355 is a low noise, low drift, low power, 3-axis mems accelerometer.

More info in the [product webpage](https://www.analog.com/en/products/ADXL355.html) and in its [datasheet](https://www.analog.com/media/en/technical-documentation/data-sheets/adxl354_adxl355.pdf).

## Minimum Supported Rust Version (MSRV)

This crate requires Rust nightly newer than `nightly-2022-11-22`, due to requiring support for
`async fn` in traits (AFIT), which is not stable yet.

Keep in mind Rust nightlies can make backwards-incompatible changes to unstable features
at any time.

## Usage

This driver relies on an implementation of either `embedded_hal_async::spi::SpiDevice` or `embedded_hal_async::i2c::I2c` traits by a HAL.
At the time of this writing, it seems that only the [embassy-hal](https://embassy.dev/) provides one.

Add this crate as a dependency in your `Cargo.toml` file:

```toml
[dependencies.adi-adxl355-async]
version = "<version>"
feature = ["defmt"] #optionally activate defmt feature
```

Basic usage of the driver should look like this:

```rust ignore
use adi_adxl355_async::{Adxl355, Config as Adxl355Config};

//The ADXL355 driver supports SPI and I2C protocols:

// After setting up a embedded_hal_async::spi::SpiDevice implementation named spi_dev,
// you can create an ADXL355 using the SPI protocol:
let mut acc = Adxl355::new_spi_with_config(spi_dev, Adxl355Config::default()).await?;
// After setting up a embedded_hal_async::i2c::I2c implementation named i2c_dev,
// you can create an ADXL355 using the I2C protocol, its I2C address must also be provided:
let mut acc = Adxl355::new_i2c_with_config(i2c_dev, device_address,Adxl355Config::default()).await?;

let dev_id = acc.get_device_id().await?;

println!("Device ID is {:X}", dev_id);

acc.set_measurement_mode().await?;

let raw_data = acc.get_raw_accel_sample().await?;

println!(
    "Raw data x={}, y={}, z={}",
    raw_data.x, raw_data.y, raw_data.z
);

let norm_data = acc.get_accel_sample().await?;

println!(
    "Normalized data x={}, y={}, z={}",
    norm_data.x, norm_data.y, norm_data.z
);
```

## Examples

Examples running on the Nordic nRF52840 are available inside the repository as a workspace member.

## License

This project is licensed under the [Apache 2.0 license](./LICENSE).

## Copyright Information

Copyright © 2023 Analog Devices, Inc. All Rights Reserved. This documentation is
proprietary to Analog Devices, Inc. and its licensors.

## Disclaimer

Analog Devices, Inc. (“Analog Devices”) reserves the right to change this product without
prior notice. Information furnished by Analog Devices is believed to be accurate and
reliable at the time it is released. The Software, its Documentation and any associated
manuals are provided on an AS IS basis without any representation, warranty, or liability
of any kind. No license is granted by implication or otherwise under the patent or other
rights of Analog Devices.
