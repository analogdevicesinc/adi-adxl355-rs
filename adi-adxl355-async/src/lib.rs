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
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![doc = include_str!("../README.md")]

pub use adi_adxl355::config::*;
pub use adi_adxl355::interrupt::*;
pub use adi_adxl355::modes::*;
pub use adi_adxl355::register::*;
pub use adi_adxl355::status::*;
pub use adi_adxl355::DeviceI2cAddress;
pub use adi_adxl355::DEVICE_ID;
use embedded_hal::spi;
use embedded_hal_async::i2c::I2c;
use embedded_hal_async::spi::SpiDevice;
use micromath::vector::{F32x3, I32x3};

/// ADXL355 async driver.
pub struct Adxl355<PROTOCOL: Protocol> {
    pub protocol: PROTOCOL,
    shadow_values: [u8; 5],
    range_scale_factor: f32,
}

impl<SPI> Adxl355<SpiProtocol<SPI>>
where
    SPI: SpiDevice,
{
    /// Create a new ADXL355 driver from given SPI peripheral in unknown state.
    pub async fn new_spi(spi: SPI) -> Result<Self, Error<SPI::Error>> {
        let mut adxl355 = Adxl355 {
            protocol: SpiProtocol { spi },
            shadow_values: [0; 5],
            range_scale_factor: 0.0,
        };
        adxl355.init_shadow_values().await?;
        adxl355.range_scale_factor = adxl355.get_range().await?.scale_factor();
        Ok(adxl355)
    }

    /// Create a new ADXL355 driver from given SPI peripheral in the given config.
    /// The ADXL355 is reset before the config is applied to make sure it is in standby mode.
    pub async fn new_spi_with_config(spi: SPI, config: Config) -> Result<Self, Error<SPI::Error>> {
        let mut adxl355 = Self::new_spi(spi).await?;
        adxl355.configure(config).await?;
        Ok(adxl355)
    }
}

impl<I2C> Adxl355<I2cProtocol<I2C>>
where
    I2C: I2c,
{
    /// Create a new ADXL355 driver from given I2C peripheral in unknown state.
    pub async fn new_i2c(i2c: I2C, address: DeviceI2cAddress) -> Result<Self, Error<I2C::Error>> {
        let mut adxl355 = Adxl355 {
            protocol: I2cProtocol {
                i2c,
                address: address as u8,
            },
            shadow_values: [0; 5],
            range_scale_factor: 0.0,
        };
        adxl355.init_shadow_values().await?;
        adxl355.range_scale_factor = adxl355.get_range().await?.scale_factor();
        Ok(adxl355)
    }

    /// Create a new ADXL355 driver from given I2C peripheral in the given config.
    /// The ADXL355 is reset before the config is applied to make sure it is in standby mode.
    pub async fn new_i2c_with_config(
        i2c: I2C,
        address: DeviceI2cAddress,
        config: Config,
    ) -> Result<Self, Error<I2C::Error>> {
        let mut adxl355 = Self::new_i2c(i2c, address).await?;
        adxl355.configure(config).await?;
        Ok(adxl355)
    }
}

impl<PROTOCOL> Adxl355<PROTOCOL>
where
    PROTOCOL: Protocol,
{
    /// Configure the ADXL355.
    /// The ADXL355 is reset before the config is applied to make sure it is in standby mode.
    pub async fn configure(
        &mut self,
        config: Config,
    ) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        let dev_id = self.get_device_id().await?;
        if dev_id != DEVICE_ID {
            return Err(Error::BadDeviceId(dev_id));
        }

        self.reset().await?;

        let mode = self.get_mode().await?;

        if !mode.is_in_standby() {
            return Err(Error::DeviceRunning);
        }

        self.protocol
            .write_register(Register::FILTER, (config.hpf.val() << 4) | config.odr.val())
            .await?;
        let mut range_register_value = self.protocol.read_register(Register::RANGE).await?;
        range_register_value |= config.range.val();
        self.protocol
            .write_register(Register::RANGE, range_register_value)
            .await?;
        self.range_scale_factor = config.range.scale_factor();

        Ok(())
    }

    /// Get device [`Status`]
    pub async fn get_status(&mut self) -> Result<Status, Error<PROTOCOL::ProtocolError>> {
        let status_val = self.protocol.read_register(Register::STATUS).await?;
        Ok(Status(status_val))
    }

    /// Get the device ID, `0xED` is expected.
    pub async fn get_device_id(&mut self) -> Result<u8, Error<PROTOCOL::ProtocolError>> {
        self.protocol.read_register(Register::PARTID).await
    }

    /// Get the product revision ID.
    pub async fn get_revision_id(&mut self) -> Result<u8, Error<PROTOCOL::ProtocolError>> {
        self.protocol.read_register(Register::REVID).await
    }

    /// Reset the device and make sure the NVM is loaded correctly.
    pub async fn reset(&mut self) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        self.protocol.write_register(Register::RESET, 0x52).await?;

        let mut nb_of_retries = 255;
        let mut nvm_busy = self.get_status().await?.is_nvm_busy();

        while nvm_busy && nb_of_retries > 0 {
            nvm_busy = self.get_status().await?.is_nvm_busy();
            nb_of_retries -= 1;
        }

        if nb_of_retries == 0 || nvm_busy {
            return Err(Error::SoftResetFailed);
        }

        let shadow_values = self.get_shadow_values().await?;

        if self.shadow_values == shadow_values {
            Ok(())
        } else {
            Err(Error::SoftResetFailed)
        }
    }

    /// Get the current [`Mode`] of the device.
    pub async fn get_mode(&mut self) -> Result<Mode, Error<PROTOCOL::ProtocolError>> {
        self.protocol
            .read_register(Register::POWER_CTL)
            .await
            .map(Mode)
    }

    /// Set the [`Mode`] of the device.
    pub async fn set_mode(&mut self, mode: Mode) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        self.protocol
            .write_register(Register::POWER_CTL, mode.0)
            .await
    }

    /// Get the current [`Range`] of the device.
    pub async fn get_range(&mut self) -> Result<Range, Error<PROTOCOL::ProtocolError>> {
        self.protocol
            .read_register(Register::RANGE)
            .await
            .map(Range::from)
    }

    /// Get the current [`Odr`] of the device.
    pub async fn get_odr_lpf(&mut self) -> Result<Odr, Error<PROTOCOL::ProtocolError>> {
        self.protocol
            .read_register(Register::FILTER)
            .await
            .map(Odr::from)
    }

    /// Get the current [`HpfCorner`] of the device.
    pub async fn get_hpf_corner(&mut self) -> Result<HpfCorner, Error<PROTOCOL::ProtocolError>> {
        self.protocol
            .read_register(Register::FILTER)
            .await
            .map(HpfCorner::from)
    }

    /// Get the current [`Config`] of the device.
    pub async fn get_config(&mut self) -> Result<Config, Error<PROTOCOL::ProtocolError>> {
        let range = self.get_range().await?;

        let filter_val = self.protocol.read_register(Register::FILTER).await?;
        let odr = Odr::from(filter_val);
        let hpf = HpfCorner::from(filter_val);

        Ok(Config { range, odr, hpf })
    }

    /// Set device in measurement mode.
    pub async fn set_measurement_mode(&mut self) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        let mode = self.protocol.read_register(Register::POWER_CTL).await?;
        self.protocol
            .write_register(Register::POWER_CTL, mode & !Mode::STANDBY_BIT)
            .await
    }

    /// Set device in standby mode.
    pub async fn set_standby_mode(&mut self) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        let mode = self.protocol.read_register(Register::POWER_CTL).await?;
        self.protocol
            .write_register(Register::POWER_CTL, mode | Mode::STANDBY_BIT)
            .await
    }

    /// Get raw temperature value.
    pub async fn get_temperature_raw(&mut self) -> Result<u16, Error<PROTOCOL::ProtocolError>> {
        let mut buf: [u8; 2] = [0; 2];

        self.protocol
            .read_multiple_registers(Register::TEMP2, &mut buf)
            .await?;

        Ok((((buf[0] & 0x0F) as u16) << 8) | ((buf[1] as u16) & 0xFF))
    }

    // Get temperature in Celsius.
    pub async fn get_temparature(&mut self) -> Result<f32, Error<PROTOCOL::ProtocolError>> {
        let raw_temp = self.get_temperature_raw().await?;
        Ok(((((raw_temp as i32) - 1885i32) as f32) / (-9.05f32)) + 25.0f32)
    }

    /// Get raw acceleration values.
    pub async fn get_raw_accel_sample(&mut self) -> Result<I32x3, Error<PROTOCOL::ProtocolError>> {
        let mut buf: [u8; 9] = [0; 9];
        self.protocol
            .read_multiple_registers(Register::XDATA3, &mut buf)
            .await?;

        let mut sample = I32x3 { x: 0, y: 0, z: 0 };

        Self::acc_buf_to_raw_sample(&buf, &mut sample);

        Ok(sample)
    }

    /// Get acceleration values in g.
    pub async fn get_accel_sample(&mut self) -> Result<F32x3, Error<PROTOCOL::ProtocolError>> {
        let mut buf: [u8; 9] = [0; 9];
        self.protocol
            .read_multiple_registers(Register::XDATA3, &mut buf)
            .await?;

        let mut sample = F32x3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        Self::acc_buf_to_sample(&buf, &mut sample, self.range_scale_factor);

        Ok(sample)
    }

    /// Enable device self test feature.
    pub async fn start_self_test(&mut self) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        self.protocol
            .write_register(Register::SELF_TEST, 0x03)
            .await
    }

    /// Disable device self test feature.
    pub async fn stop_self_test(&mut self) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        self.protocol
            .write_register(Register::SELF_TEST, 0x00)
            .await
    }

    /// Apply offset trims for raw axis data after all other signal processing.
    /// The offset trim value is removed from the axis data bits\[19:4\].
    pub async fn set_offset_trims(
        &mut self,
        offset_x: i16,
        offset_y: i16,
        offset_z: i16,
    ) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        let mut buf = [
            Register::OFFSET_X_H.addr(),
            ((offset_x >> 8) & 0xFF) as u8,
            ((offset_x) & 0xFF) as u8,
            ((offset_y >> 8) & 0xFF) as u8,
            ((offset_y) & 0xFF) as u8,
            ((offset_z >> 8) & 0xFF) as u8,
            ((offset_z) & 0xFF) as u8,
        ];
        self.protocol.write_multiple_registers(&mut buf).await
    }

    /// Get the number of data samples stored in the FIFO
    pub async fn get_nb_samples_in_fifo(&mut self) -> Result<u8, Error<PROTOCOL::ProtocolError>> {
        let nb_samples = self.protocol.read_register(Register::FIFO_ENTRIES).await? & 0x7F;
        Ok(nb_samples)
    }

    /// Set the maximum number of samples the fifo will store.
    /// Value must range from 1 to 96.
    pub async fn set_nb_max_samples_in_fifo(
        &mut self,
        fifo_samples: u8,
    ) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        if !(1..=96).contains(&fifo_samples) {
            return Err(Error::InvalidMaxNbFifoSamples);
        }

        self.protocol
            .write_register(Register::FIFO_SAMPLES, fifo_samples)
            .await
    }

    /// Get data from the fifo into provided array of raw accelerometer samples.
    /// The return value indicates how many samples were retrieved.
    pub async fn get_fifo_raw_data(
        &mut self,
        data: &mut [I32x3],
    ) -> Result<usize, Error<PROTOCOL::ProtocolError>> {
        let mut nb_available_samples = self.get_nb_samples_in_fifo().await?;
        if nb_available_samples < 3 {
            return Err(Error::NotEnoughData);
        }
        let data_len = data.len();
        let mut nb_samples_retrieved: usize = 0;

        let mut buf = [0u8; 9];
        self.protocol
            .read_multiple_registers(Register::FIFO_DATA, &mut buf[..3])
            .await?;
        nb_available_samples -= 1;

        // Make sure first value is from X axis
        while buf[2] & 0x01 == 0 {
            if nb_available_samples < 3 {
                return Err(Error::NotEnoughData);
            }

            self.protocol
                .read_multiple_registers(Register::FIFO_DATA, &mut buf[..3])
                .await?;
            nb_available_samples -= 1;
        }

        self.protocol
            .read_multiple_registers(Register::FIFO_DATA, &mut buf[3..])
            .await?;
        nb_available_samples -= 2;

        Self::acc_buf_to_raw_sample(&buf, &mut data[0]);
        nb_samples_retrieved += 1;

        while nb_available_samples >= 3 && data_len > nb_samples_retrieved {
            self.protocol
                .read_multiple_registers(Register::FIFO_DATA, &mut buf)
                .await?;
            nb_available_samples -= 3;

            Self::acc_buf_to_raw_sample(&buf, &mut data[nb_samples_retrieved]);
            nb_samples_retrieved += 1;
        }

        Ok(nb_samples_retrieved)
    }

    /// Get data from the fifo into provided array of accelerometer samples in g.
    /// The return value indicates how many samples were retrieved.
    pub async fn get_fifo_data(
        &mut self,
        data: &mut [F32x3],
    ) -> Result<usize, Error<PROTOCOL::ProtocolError>> {
        let mut nb_available_samples = self.get_nb_samples_in_fifo().await?;
        if nb_available_samples < 3 {
            return Err(Error::NotEnoughData);
        }
        let data_len = data.len();
        let mut nb_samples_retrieved: usize = 0;

        let mut buf = [0u8; 9];
        self.protocol
            .read_multiple_registers(Register::FIFO_DATA, &mut buf[..3])
            .await?;
        nb_available_samples -= 1;

        // Make sure first value is from X axis
        while buf[2] & 0x01 == 0 {
            if nb_available_samples < 3 {
                return Err(Error::NotEnoughData);
            }

            self.protocol
                .read_multiple_registers(Register::FIFO_DATA, &mut buf[..3])
                .await?;
            nb_available_samples -= 1;
        }

        self.protocol
            .read_multiple_registers(Register::FIFO_DATA, &mut buf[3..])
            .await?;
        nb_available_samples -= 2;

        Self::acc_buf_to_sample(&buf, &mut data[0], self.range_scale_factor);
        nb_samples_retrieved += 1;

        while nb_available_samples >= 3 && data_len > nb_samples_retrieved {
            self.protocol
                .read_multiple_registers(Register::FIFO_DATA, &mut buf)
                .await?;
            nb_available_samples -= 3;

            Self::acc_buf_to_sample(
                &buf,
                &mut data[nb_samples_retrieved],
                self.range_scale_factor,
            );
            nb_samples_retrieved += 1;
        }

        Ok(nb_samples_retrieved)
    }

    /// Enable activity detection for selected axes
    pub async fn enable_activity_detection(
        &mut self,
        x: bool,
        y: bool,
        z: bool,
    ) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        let mut act_en = 0;
        if x {
            act_en += 0x01;
        }
        if y {
            act_en += 0x02;
        }
        if z {
            act_en += 0x04;
        }

        self.protocol.write_register(Register::ACT_EN, act_en).await
    }

    /// Set threshold for activity detection. The acceleration magnitude must be greater
    /// than the value in ACT_THRESH to trigger the activity counter.
    /// The significance of the threshold matches the significance of Bits\[18:3\]
    /// of the accelerometer's axes data.
    pub async fn set_activity_threshold(
        &mut self,
        threshold: u16,
    ) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        self.protocol
            .write_multiple_registers(&mut [
                Register::ACT_THRESH_H.addr(),
                ((threshold >> 8) & 0xFF) as u8,
                (threshold & 0xFF) as u8,
            ])
            .await
    }

    /// Set number of consecutive events above threshold required to detect activity.
    pub async fn set_activity_count(
        &mut self,
        count: u8,
    ) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        self.protocol
            .write_register(Register::ACT_COUNT, count)
            .await
    }

    /// Configure interrupt pins INT1 and INT2.
    pub async fn configure_interrupt_pins(
        &mut self,
        conf: InterruptConfig,
    ) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        self.protocol
            .write_register(Register::INT_MAP, conf.0)
            .await
    }

    /// Set interrupt pins polarity.
    pub async fn set_interrupt_polarity(
        &mut self,
        polarity: InterruptPolarity,
    ) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        let mut buf = self.protocol.read_register(Register::RANGE).await?;

        match polarity {
            InterruptPolarity::ActiveHigh => {
                buf |= 0x40;
            }
            InterruptPolarity::ActiveLow => {
                buf &= !0x40;
            }
        }

        self.protocol.write_register(Register::RANGE, buf).await
    }

    /// Set I2C speed mode.
    pub async fn set_i2c_speed(
        &mut self,
        mode: I2cSpeedMode,
    ) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        let mut buf = self.protocol.read_register(Register::RANGE).await?;

        match mode {
            I2cSpeedMode::HighSpeed => {
                buf |= 0x80;
            }
            I2cSpeedMode::Fast => {
                buf &= !0x80;
            }
        }

        self.protocol.write_register(Register::RANGE, buf).await
    }

    /// Set the synchronization mode the device will operate in. See options in [`ExternalSyncMode`].
    pub async fn set_synchronization_mode(
        &mut self,
        mode: ExternalSyncMode,
    ) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        self.protocol
            .write_register(Register::SYNC, mode as u8)
            .await
    }

    /// Converts 9 bytes to raw acceleration sample.
    fn acc_buf_to_raw_sample(buf: &[u8; 9], sample: &mut I32x3) {
        sample.x =
            (((buf[0] as i32) << 24) | ((buf[1] as i32) << 16) | ((buf[2] & 0xF0) as i32) << 8)
                >> 12;
        sample.y =
            (((buf[3] as i32) << 24) | ((buf[4] as i32) << 16) | ((buf[5] & 0xF0) as i32) << 8)
                >> 12;
        sample.z =
            (((buf[6] as i32) << 24) | ((buf[7] as i32) << 16) | ((buf[8] & 0xF0) as i32) << 8)
                >> 12;
    }

    /// Converts 9 bytes to acceleration sample in g using the scale factor provided.
    fn acc_buf_to_sample(buf: &[u8; 9], sample: &mut F32x3, scale_factor: f32) {
        sample.x =
            ((((buf[0] as i32) << 24) | ((buf[1] as i32) << 16) | ((buf[2] & 0xF0) as i32) << 8)
                >> 12) as f32
                * scale_factor;
        sample.y =
            ((((buf[3] as i32) << 24) | ((buf[4] as i32) << 16) | ((buf[5] & 0xF0) as i32) << 8)
                >> 12) as f32
                * scale_factor;
        sample.z =
            ((((buf[6] as i32) << 24) | ((buf[7] as i32) << 16) | ((buf[8] & 0xF0) as i32) << 8)
                >> 12) as f32
                * scale_factor;
    }

    /// Read the shadow registers initial values.
    async fn init_shadow_values(&mut self) -> Result<(), Error<PROTOCOL::ProtocolError>> {
        self.protocol
            .read_multiple_registers(Register::SHADOW_REG1, &mut self.shadow_values)
            .await?;
        Ok(())
    }

    /// Get the shdow registers' values.
    async fn get_shadow_values(&mut self) -> Result<[u8; 5], Error<PROTOCOL::ProtocolError>> {
        let mut shadow_values = [0u8; 5];
        self.protocol
            .read_multiple_registers(Register::SHADOW_REG1, &mut shadow_values)
            .await?;
        Ok(shadow_values)
    }
}

pub trait Protocol {
    type ProtocolError: core::fmt::Debug;

    /// Read a single register.
    async fn read_register(&mut self, register: Register)
        -> Result<u8, Error<Self::ProtocolError>>;

    /// Read `buf.len()` registers from the `start_register` address included.
    async fn read_multiple_registers(
        &mut self,
        start_register: Register,
        buf: &mut [u8],
    ) -> Result<(), Error<Self::ProtocolError>>;

    /// Write a single register.
    async fn write_register(
        &mut self,
        register: Register,
        buf: u8,
    ) -> Result<(), Error<Self::ProtocolError>>;

    /// Write `buf.len() - 1` registers, the first byte must be the address of the start register.
    async fn write_multiple_registers(
        &mut self,
        buf: &mut [u8],
    ) -> Result<(), Error<Self::ProtocolError>>;
}

pub struct SpiProtocol<SPI> {
    spi: SPI,
}

impl<SPI> Protocol for SpiProtocol<SPI>
where
    SPI: SpiDevice,
{
    type ProtocolError = SPI::Error;

    /// Read a single register.
    async fn read_register(
        &mut self,
        register: Register,
    ) -> Result<u8, Error<Self::ProtocolError>> {
        let mut buf = [0u8; 1];
        self.spi
            .transaction(&mut [
                spi::Operation::Write(&[register.addr() << 1 | 0x01]),
                spi::Operation::Read(&mut buf),
            ])
            .await
            .map_err(Error::Protocol)?;
        Ok(buf[0])
    }

    /// Read `buf.len()` registers from the `start_register` address included.
    async fn read_multiple_registers(
        &mut self,
        start_register: Register,
        buf: &mut [u8],
    ) -> Result<(), Error<Self::ProtocolError>> {
        self.spi
            .transaction(&mut [
                spi::Operation::Write(&[(start_register.addr() << 1) | 0x01]),
                spi::Operation::Read(buf),
            ])
            .await
            .map_err(Error::Protocol)?;
        Ok(())
    }

    /// Write a single register.
    async fn write_register(
        &mut self,
        register: Register,
        buf: u8,
    ) -> Result<(), Error<Self::ProtocolError>> {
        self.spi
            .write(&[register.addr() << 1, buf])
            .await
            .map_err(Error::Protocol)?;
        Ok(())
    }

    /// Write `buf.len() - 1` registers, the first byte must be the address of the register.
    async fn write_multiple_registers(
        &mut self,
        buf: &mut [u8],
    ) -> Result<(), Error<Self::ProtocolError>> {
        buf[0] <<= 1;
        self.spi.write(buf).await.map_err(Error::Protocol)?;
        Ok(())
    }
}

pub struct I2cProtocol<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> Protocol for I2cProtocol<I2C>
where
    I2C: I2c,
{
    type ProtocolError = I2C::Error;

    /// Read a single register.
    async fn read_register(
        &mut self,
        register: Register,
    ) -> Result<u8, Error<Self::ProtocolError>> {
        let mut buf = [0u8; 1];
        self.i2c
            .write_read(self.address, &[register.addr()], &mut buf)
            .await
            .map_err(Error::Protocol)?;
        Ok(buf[0])
    }

    /// Read `buf.len()` registers from the `start_register` address included.
    async fn read_multiple_registers(
        &mut self,
        start_register: Register,
        buf: &mut [u8],
    ) -> Result<(), Error<Self::ProtocolError>> {
        self.i2c
            .write_read(self.address, &[start_register.addr()], buf)
            .await
            .map_err(Error::Protocol)?;
        Ok(())
    }

    /// Write a single register.
    async fn write_register(
        &mut self,
        register: Register,
        buf: u8,
    ) -> Result<(), Error<Self::ProtocolError>> {
        self.i2c
            .write(self.address, &[register.addr(), buf])
            .await
            .map_err(Error::Protocol)?;
        Ok(())
    }

    /// Write `buf.len() - 1` registers, the first byte must be the address of the register.
    async fn write_multiple_registers(
        &mut self,
        buf: &mut [u8],
    ) -> Result<(), Error<Self::ProtocolError>> {
        self.i2c
            .write(self.address, buf)
            .await
            .map_err(Error::Protocol)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Driver errors
pub enum Error<ProtocolError: core::fmt::Debug> {
    /// Protocol error.
    Protocol(ProtocolError),

    /// Device ID is not 0xED.
    BadDeviceId(u8),

    /// Device is in measure mode and cannot be configured.
    DeviceRunning,

    /// Software reset failed to reset device properly.
    SoftResetFailed,

    /// The max number of samples to store in the fifo is invalid. Must range from 1 to 96.
    InvalidMaxNbFifoSamples,

    /// The fifo does not contain enough data to get one sample with the 3-axis values.
    NotEnoughData,
}
