// Copyright © 2023 Analog Devices, Inc. All Rights Reserved. This software is
// proprietary to Analog Devices, Inc. and its licensors.
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Result;
use linux_embedded_hal::{
    gpio_cdev::{Chip, EventRequestFlags, LineEventHandle, LineRequestFlags},
    spidev::{SpiModeFlags, SpidevOptions},
    Spidev,
};

use adi_adxl355::{Adxl355, Config as Adxl355Config, Protocol, Register as Adxl355Register};

const SPI_PATH: &str = "/dev/spidev0.0";
const GPIO_PATH: &str = "/dev/gpiochip0";
const DRDY_PIN: u32 = 27;

fn main() -> Result<()> {
    let mut spi_dev = Spidev::open(SPI_PATH)?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(10_000_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi_dev.configure(&options)?;
    let mut chip = Chip::new(GPIO_PATH)?;
    let drdy = chip.get_line(DRDY_PIN)?.events(
        LineRequestFlags::INPUT,
        EventRequestFlags::RISING_EDGE,
        "adxl355 drdy",
    )?;

    get_adxl355_data(spi_dev, drdy)
}

fn get_adxl355_data(spi_dev: Spidev, mut drdy: LineEventHandle) -> Result<()> {
    let mut acc = Adxl355::new_spi_with_config(spi_dev, Adxl355Config::default())?;

    let dev_id = acc.get_device_id()?;
    println!("Device ID is {:X}", dev_id);

    let status = acc.get_status()?;
    println!("Device status: {:?}", status);

    let rev_id = acc.protocol.read_register(Adxl355Register::REVID)?;
    println!("Device Revision ID is {:X}", rev_id);

    acc.set_measurement_mode()?;

    loop {
        let _data_ready = drdy.get_event()?;
        let raw_data = acc.get_raw_accel_sample()?;
        println!(
            "Raw data x={}, y={}, z={}",
            raw_data.x, raw_data.y, raw_data.z
        );
        let norm_data = acc.get_accel_sample()?;
        println!(
            "Normalized data x={}, y={}, z={}",
            norm_data.x, norm_data.y, norm_data.z
        );
    }
}
