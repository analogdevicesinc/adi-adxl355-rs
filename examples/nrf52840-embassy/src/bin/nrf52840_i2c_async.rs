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
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{self, Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::gpiote::{self, InputChannel};
use embassy_nrf::peripherals::TWISPI0;
use embassy_nrf::{bind_interrupts, twim};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::i2c::Error as I2cError;
use embedded_hal_async::i2c::I2c;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use adi_adxl355_async::{
    Adxl355, Config as Adxl355Config, DeviceI2cAddress, Error as Adxl355Error, Protocol,
    Register as Adxl355Register,
};

bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<TWISPI0>;
});

static I2C_BUS: StaticCell<Mutex<ThreadModeRawMutex, twim::Twim<TWISPI0>>> = StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let led_pin = p.P0_13;
    let i2c_peripheral = p.TWISPI0;
    let asel_pin = p.P0_28;
    let sda_pin = p.P0_30;
    let scl_pin = p.P0_31;
    let drdy_pin = p.P0_04;
    let drdy_gpiote_channel = p.GPIOTE_CH0;

    let led = Output::new(led_pin, Level::High, OutputDrive::HighDrive);

    // This is just for the purpose of the example, the ADXL355 ASEL pin could be grounded instead of using a GPIO.
    let mut adxl355_asel = Output::new(asel_pin, Level::Low, OutputDrive::Standard);
    adxl355_asel.set_low();
    let device_address = if adxl355_asel.is_set_low() {
        DeviceI2cAddress::AselPinLow
    } else {
        DeviceI2cAddress::AselPinHigh
    };

    let mut config = twim::Config::default();
    // This limits the ADXL355 ODR to 200Hz max.
    config.frequency = twim::Frequency::K100;
    // Internal pullups for SCL and SDA must be enabled.
    config.scl_pullup = true;
    config.sda_pullup = true;

    let i2c = twim::Twim::new(i2c_peripheral, Irqs, sda_pin, scl_pin, config);
    let i2c_bus = Mutex::<ThreadModeRawMutex, _>::new(i2c);
    let i2c_bus = I2C_BUS.init(i2c_bus);
    let i2c_dev = I2cDevice::new(i2c_bus);

    let drdy = InputChannel::new(
        drdy_gpiote_channel,
        Input::new(drdy_pin, Pull::Down),
        embassy_nrf::gpiote::InputChannelPolarity::LoToHi,
    );

    get_adxl355_data(led, i2c_dev, device_address, drdy)
        .await
        .unwrap();
}

async fn get_adxl355_data(
    mut led: impl OutputPin<Error = Infallible>,
    i2c_dev: impl I2c<Error = impl I2cError>,
    device_address: DeviceI2cAddress,
    drdy: InputChannel<'_, impl gpiote::Channel, impl gpio::Pin>,
) -> Result<(), Adxl355Error<impl I2cError>> {
    let mut acc =
        Adxl355::new_i2c_with_config(i2c_dev, device_address, Adxl355Config::default()).await?;

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
