# ADXL355 on Raspberry Pi

## Hardware Requirements

For this project, we are using a Raspberry Pi 4 running the Raspberry Pi OS 64-bit and ADI's EVAL-ADXL355 accelerometer evaluation board.
The two boards are wired to enable SPI communications between them.

### Wiring EVAL-ADXL355 to the Raspberry Pi 4

We connected the ADXL355 directly to the Raspberry Pi 4's [40-pin header](https://www.raspberrypi.com/documentation/computers/os.html#gpio-and-the-40-pin-header).

| Function | Raspberry Pi 4 pin | EVAL-ADXL355 pin | Description                         |
| :------- | -----------------: | ---------------: | ----------------------------------- |
| VDDIO    |          3V3 Power |            P1.01 | Digital Interface Supply Voltage    |
| VDD      |          3V3 Power |            P1.03 | Supply Voltage                      |
| GND      |             Ground |            P1.05 | Ground                              |
| DRDY     |             GPIO27 |            P1.06 | Data Ready Pin                      |
| CS       |                CE0 |            P2.02 | Chip Select for SPI                 |
| SCLK     |               SCLK |            P2.04 | Serial Communications Clock for SPI |
| MISO     |               MISO |            P2.05 | Master Input, Slave Output for SPI  |
| MOSI     |               MOSI |            P2.06 | Master Output, Slave Input for SPI  |

Note that on the Raspberry Pi 4, any GPIO pin can be used for the DRDY function.
If you decide to wire the DRDY line differently, you'll have to adjust the pin in the firmware.

We also use GPIO17 as a fake Chip Select pin because the ADXL355 driver expect one to drive it, however the real chip select is on CE0 and is driven by the Linux SPI driver. You can change the pin in the firmware for any unused pin.

## Software requirements

### Rust and build tools

You can install Rust following instructions from the [rust-lang website](https://www.rust-lang.org/tools/install)
On Unix-like OS, you can use this command:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you plan on building the application on your machine, you'll need to have the Arm GNU Toolchain installed and add ARM64 Linux platform support to your Rust tools.

The Arm GNU Toolchain can be found [here](https://developer.arm.com/downloads/-/gnu-a), use version 10.2 for your host platform. Version 10.2 is the latest toolchain supported by the Raspberry Pi OS at the time of this writing. You may want to add the bin folder to your path.
On Ubuntu, you can instead install the toolchain with apt `sudo apt install -y gcc-aarch64-linux-gnu`, but note that the binaries will have a different naming convention.

ARM64 Linux platform support can be added to Rust with this command:

```sh
rustup target add aarch64-unknown-linux-gnu
```

### Activating the SPI interface on the Raspberry Pi

Using the Desktop GUI, go to `Pi Start Menu` > `Preferences` > `Raspberry Pi Configuration`, in the `Interfaces` tab, activate `SPI`.

Using the terminal, run `sudo raspi-config`, in `Interface Options`, select `SPI` and hit enter.

### Allow use of SPI and GPIO char dev on the Raspberry Pi

On the Raspberry Pi, add your username to the `gpio` and `spi` group:

```bash
sudo usermod -a -G spi <user>
sudo usermod -a -G gpio <user>
```

## Running the application

If you cloned this project on a Raspberry Pi, then use `cargo run --release` to simply build and run it.

If you want to cross compile this project on your machine before running the application on the Raspberry Pi, build with `cargo build --release --target aarch64-unknown-linux-gnu --config target.aarch64-unknown-linux-gnu.linker=\"aarch64-none-linux-gnu-gcc\"`.
This command assumes that you installed the Arm GNU Toolchain and that `aarch64-none-linux-gnu-gcc` is in your path. You'll have to adjust its location and/or name if needed. For example, if you installed the toolchain from apt on Ubuntu, the command is `cargo build --release --target aarch64-unknown-linux-gnu --config target.aarch64-unknown-linux-gnu.linker=\"aarch64-linux-gnu-gcc\"`
Note that on Windows, in PowerShell, the escape character for the double quotes is a backtick instead of a backslash.

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
