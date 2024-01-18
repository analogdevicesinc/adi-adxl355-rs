# Examples running on a nRF52840 for the ADXL355 accelerometer drivers

This cargo package contains three binaries, demonstrating how to use the ADXL355 drivers, one using the blocking SPI protocol, another using the async SPI protocol, and another using the async I2C/TWI protocol.

## Hardware requirement

For these examples we used the BMD-340-EVAL which features the Nordic nRF52840 RF System on Chip.

The BMD-340 user guide is available [here](https://www.u-blox.com/sites/default/files/BMD-34-38-EVAL_UserGuide_UBX-19033356.pdf), and the data sheet is available [here](https://www.u-blox.com/sites/default/files/BMD-340_DataSheet_UBX-19033353.pdf).

You could use this project with any eval board or development kit containing the Nordic nRF52840 SoC, but you may have to adjust the pins inside the example code.

The BMD-340-EVAL also features an on-board programming and debug interface, the SEGGER J-Link-OB which is supported by [probe-rs](https://probe.rs/), the chosen toolkit to load and debug the firmware.
If you use another eval board, you'll have to make sure your debug probe/interface is either supported by `probe-rs` or that you use something else for loading and debugging the firmware (e.g. OpenOCD and GDB).

### Wiring EVAL-ADXL355 to BMD-340-EVAL for SPI communications

| Function | BMD-340-EVAL pin | EVAL-ADXL355 pin | Description                         |
| :------- | ---------------: | ---------------: | ----------------------------------- |
| VDDIO    |            VSHLD |            P1.01 | Digital Interface Supply Voltage    |
| VDD      |            VSHLD |            P1.03 | Supply Voltage                      |
| GND      |              GND |            P1.05 | Ground                              |
| DRDY     |            P0.04 |            P1.06 | Data Ready Pin                      |
| CS       |            P0.31 |            P2.02 | Chip Select for SPI                 |
| SCLK     |            P0.29 |            P2.04 | Serial Communications Clock for SPI |
| MISO     |            P0.28 |            P2.05 | Master Input, Slave Output for SPI  |
| MOSI     |            P0.30 |            P2.06 | Master Output, Slave Input for SPI  |

Note that on the BMD340-EVAL, any pin in the range P0.00-P0.31, and P1.00-P1.15 can be used for SPI communications.
If you decide to wire the ADXL355 to the BMD-340 differently, you'll have to adjust the pins in the example code.

### Wiring EVAL-ADXL355 to BMD-340-EVAL for I2C communications

| Function | BMD-340-EVAL pin | EVAL-ADXL355 pin | Description                          |
| :------- | ---------------: | ---------------: | ------------------------------------ |
| VDDIO    |            VSHLD |            P1.01 | Digital Interface Supply Voltage     |
| VDD      |            VSHLD |            P1.03 | Supply Voltage                       |
| GND      |              GND |            P1.05 | Ground                               |
| DRDY     |            P0.04 |            P1.06 | Data Ready Pin                       |
| SCL      |            P0.31 |            P2.02 | Serial Communications Clock for I2C  |
| VSSIO    |              GND |            P2.04 | I2C Mode Enable                      |
| ASEL     |            P0.28 |            P2.05 | Alternate I2C Address Select for I2C |
| SDA      |            P0.30 |            P2.06 | Serial Data for I2C                  |

Note that on the BMD340-EVAL, any pin in the range P0.00-P0.31, and P1.00-P1.15 can be used for I2C communications.
If you decide to wire the ADXL355 to the BMD-340 differently, you'll have to adjust the pins in the example code.

### Wiring BMD-340-EVAL to host

The BMD-340-EVAL has two micro USB ports, we use J4 to power, program, and debug the example.

## Software requirements

### Rust and build tools

Rust can be installed following instructions from the [rust-lang website](https://www.rust-lang.org/tools/install).
On Unix-like OS, you can use this command:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Cross-compilation support for the Cortex-M4F architecture has to be added with:

```sh
rustup target add thumbv7em-none-eabihf
```

[flip-link](https://github.com/knurling-rs/flip-link) for stack overflow protection :

```sh
cargo install flip-link
```

We recommend installing LLVM tools (including `objdump`, `nm`, `size`) :

```sh
rustup component add llvm-tools-preview
```

And the cargo subcommands to use them:

```sh
cargo install cargo-binutils
```

### Debugging tools

We'll use utilities based on the `probe-rs` toolkit to load the examples and debug them on the device.

#### System dependencies On Ubuntu

Install `probe-rs` system dependencies:

```sh
sudo apt install -y libusb-1.0-0-dev libudev-dev
```

We also have to make sure we can access the debug probe. We'll add all the debug probes supported by `probe-rs`.

- Download this [file](https://probe.rs/files/69-probe-rs.rules) containing device rules
- Copy it in `/etc/udev/rules.d`
- Run `udevadm control --reload`
- Run `udevadm trigger`

#### USB driver on Windows

`probe-rs` does not support the Segger JLink driver. It needs to be replaced by the generic WinUSB driver using a tool like [Zadig](https://zadig.akeo.ie/#). More info in [probe-rs docs](https://probe.rs/docs/getting-started/probe-setup/).

#### cargo-embed

[cargo-embed](https://probe.rs/docs/tools/cargo-embed/) enables loading and running the firmware with `cargo embed`.
Its configuration is in `Embed.toml`. It is set to open an RTTUI to display prints and logs. It has the option to start a GDB server which is not enabled by default.

It is installed as part of `probe-rs` tools with:

```sh
cargo install probe-rs --features cli
```

#### probe-rs for VSCode

[probe-rs for VSCode](https://probe.rs/docs/tools/vscode/) with its corresponding [VSCode extension](https://marketplace.visualstudio.com/items?itemName=probe-rs.probe-rs-debugger) enables debugging the firmware right from VSCode.

It is installed as part of `probe-rs` tools with:

```sh
cargo install probe-rs --features cli
```

Then install the VSCode extension.
Its configuration is in `.vscode/launch.json`.

### Note on prints and logs when debugging

We use [defmt](https://defmt.ferrous-systems.com/) to print and log. As `defmt` and its different support crates are still unstable at this time (still in version 0.x.y), they may introduce breaking changes that would make versions running on the firmware and versions used by the debugging tools incompatible.

If logs and prints don't show up, either:

- pin the version of `defmt` used in the firmware to the one you compiled the debugging tools with
- force a re-build of the debugging tools to use the most recent version of `defmt` (e.g. `cargo install probe-run --force`)

### Running the SPI example

Using the aliases defined in `.cargo/config.toml`, it can be built with `cargo bnrf` and, if `probe-rs` is installed, it can be run with `cargo rnrfspi`.

It can also be built, loaded, and run with `cargo-embed` with `cargo embed --package nrf52840-adxl355-embassy-example --bin nrf52840_spi nrf52840`.

With `probe-rs-debugger` installed, it can be built, loaded, run and debugged from VSCode Run and Debug tab by selecting the `Debug nrf52840 SPI example` target.

### Running the I2C example

Using the aliases defined in `.cargo/config.toml`, it can be built with `cargo bnrf` and, if `probe-rs` is installed, it can be run with `cargo rnrfi2c`.

It can also be built, loaded, and run with `cargo-embed` with `cargo embed --package nrf52840-adxl355-embassy-example --bin nrf52840_i2c_async nrf52840`.

With `probe-rs-debugger` installed, it can be built, loaded, run and debugged from VSCode Run and Debug tab by selecting the `Debug nrf52840 async I2C example` target.

## License

This project is licensed under the [Apache 2.0 license](LICENSE).

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
