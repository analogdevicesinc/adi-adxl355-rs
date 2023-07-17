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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// ADXL355 mode of operation.
pub struct Mode(pub u8);

impl Mode {
    pub const DRDY_OFF_BIT: u8 = 0x04;
    pub const TEMP_OFF_BIT: u8 = 0x02;
    pub const STANDBY_BIT: u8 = 0x01;

    ///  In standby mode, the device is in a low power state, and the
    /// temperature and acceleration datapaths are not operating. In addition, digital
    /// functions, including FIFO pointers, reset. Changes to the configuration setting of the
    /// device must be made when in standby. An exception is a high-pass filter that can be
    /// changed when the device is operating.
    pub fn standby() -> Self {
        Mode(Self::STANDBY_BIT)
    }

    /// In measure mode, the temperature and acceleration datapaths are operating.
    /// No changes to the configuration can be made.
    pub fn measure() -> Self {
        Mode(0)
    }

    /// Disable temperature processing.
    pub fn with_temp_off(&mut self) -> Self {
        self.0 |= Self::TEMP_OFF_BIT;
        *self
    }

    /// Force DRDY output to 0.
    pub fn with_drdy_off(&mut self) -> Self {
        self.0 |= Self::DRDY_OFF_BIT;
        *self
    }

    /// Check if device is in standby mode.
    pub fn is_in_standby(&self) -> bool {
        self.0 & Self::STANDBY_BIT == Self::STANDBY_BIT
    }

    /// Check if temperature processing is disabled.
    pub fn is_temperature_off(&self) -> bool {
        self.0 & Self::TEMP_OFF_BIT == Self::TEMP_OFF_BIT
    }

    /// Check if the DRDY output is forced to 0 in modes where it is normally signal data ready.
    pub fn is_data_ready_off(&self) -> bool {
        self.0 & Self::DRDY_OFF_BIT == Self::DRDY_OFF_BIT
    }
}

impl Default for Mode {
    /// By default, the device is in standby mode.
    fn default() -> Self {
        Mode(Self::STANDBY_BIT)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
/// ADXL355 I2C speed mode.
pub enum I2cSpeedMode {
    Fast = 0,
    HighSpeed = 1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
/// ADXL355 external synchronization and interpolation options.
pub enum ExternalSyncMode {
    NoExtSync = 0,
    ExtSyncWithInterpolation = 0x02,
    ExtSyncExtClockNoInterpolation = 0x05,
    ExtSyncExtClockWithInterpolation = 0x06,
}
