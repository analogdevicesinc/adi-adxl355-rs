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

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::convert::Infallible;
use defmt::println;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{self, Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::gpiote::{self, InputChannel, InputChannelPolarity};
use embassy_nrf::peripherals::SPI3;
use embassy_nrf::{bind_interrupts, spim};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::Error as SpiError;
use embedded_hal_async::spi::SpiDevice as Spi;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use adi_adxl355_async::{
    Adxl355, Config as Adxl355Config, Error as Adxl355Error, Protocol, Register as Adxl355Register,
};

bind_interrupts!(struct Irqs {
    SPIM3 => spim::InterruptHandler<SPI3>;
});

static SPI_BUS: StaticCell<Mutex<ThreadModeRawMutex, spim::Spim<SPI3>>> = StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let led_pin = p.P0_13;
    let spi_peripheral = p.SPI3;
    let sck_pin = p.P0_29;
    let miso_pin = p.P0_28;
    let mosi_pin = p.P0_30;
    let chip_select_pin = p.P0_31;
    let drdy_pin = p.P0_04;
    let drdy_gpiote_channel = p.GPIOTE_CH0;

    let led = Output::new(led_pin, Level::High, OutputDrive::HighDrive);

    let mut config = spim::Config::default();
    config.frequency = spim::Frequency::M1;
    let spi = spim::Spim::new(spi_peripheral, Irqs, sck_pin, miso_pin, mosi_pin, config);
    let ncs = Output::new(chip_select_pin, Level::High, OutputDrive::Standard);
    let spi_bus = Mutex::<ThreadModeRawMutex, _>::new(spi);
    let spi_bus = SPI_BUS.init(spi_bus);
    let spi_dev = SpiDevice::new(spi_bus, ncs);

    let drdy = InputChannel::new(
        drdy_gpiote_channel,
        Input::new(drdy_pin, Pull::Down),
        InputChannelPolarity::LoToHi,
    );

    get_adxl355_data(led, spi_dev, drdy).await.unwrap();
}

async fn get_adxl355_data(
    mut led: impl OutputPin<Error = Infallible>,
    spi_dev: impl Spi<Error = impl SpiError>,
    drdy: InputChannel<'_, impl gpiote::Channel, impl gpio::Pin>,
) -> Result<(), Adxl355Error<impl SpiError>> {
    let mut acc = Adxl355::new_spi_with_config(spi_dev, Adxl355Config::default()).await?;

    let dev_id = acc.get_device_id().await?;
    println!("Device ID is {:X}", dev_id);

    let status = acc.get_status().await?;
    println!("Device status: {}", status);

    let rev_id = acc.protocol.read_register(Adxl355Register::REVID).await?;
    println!("Device Revision ID is {:X}", rev_id);

    acc.set_measurement_mode().await?;

    loop {
        let _ = led.set_high();
        drdy.wait().await;
        let _ = led.set_low();
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
    }
}
