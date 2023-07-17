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

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Accelerometer range. The ADXL355 supports the ±2 g, ±4 g, and ±8 g ranges.
pub enum Range {
    #[default]
    _2G = 0b01,
    _4G = 0b10,
    _8G = 0b11,
}

impl Range {
    /// Range configuration value as expected in the Range bits of the [`crate::register::Register::RANGE`] register.
    pub const fn val(self) -> u8 {
        self as u8
    }

    /// Range scale factor to go from the accelerometer provided integer raw value to g value.
    pub const fn scale_factor(self) -> f32 {
        match self {
            Range::_2G => 0.000_003_9,
            Range::_4G => 0.000_007_8,
            Range::_8G => 0.000_015_6,
        }
    }
}

/// Enables retrieving Range enum value from the [`crate::register::Register::RANGE`] register value.
impl From<u8> for Range {
    fn from(value: u8) -> Self {
        match value & 0x03 {
            1 => Range::_2G,
            2 => Range::_4G,
            3 => Range::_8G,
            unexpected => panic!("Unexpected value {} for range", unexpected),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Accelerometer's output data rate (ODR).
/// The low pass filter corner frequency (LPF) is configured cojointly, the LPF is 1/4th the ODR.
pub enum Odr {
    /// odr = 4000 Hz and lpf = 1000 Hz
    _4000Hz = 0,
    /// odr = 2000 Hz and lpf = 500 Hz
    _2000Hz = 1,
    /// odr = 1000 Hz and lpf = 250 Hz
    _1000Hz = 2,
    /// odr = 500 Hz and lpf = 125 Hz
    _500Hz = 3,
    /// odr = 250 Hz and lpf = 62.5 Hz
    _250Hz = 4,
    /// odr = 125 Hz and lpf = 31.25 Hz
    _125Hz = 5,
    /// odr = 62.5 Hz and lpf = 15.625 Hz
    _62_5Hz = 6,
    /// odr = 31.25 Hz and lpf = 7.813 Hz
    _31_25Hz = 7,
    /// odr = 15.625 Hz and lpf = 3.906
    _15_625Hz = 8,
    /// odr = 7.813 Hz and lpf = 1.953 Hz
    _7_813Hz = 9,
    #[default]
    /// odr = 3.906 Hz and lpf = 0.977 Hz
    _3_906Hz = 10,
}

impl Odr {
    /// ODR_LPF configuration value as expected in the ODR_LPF bits of the [`crate::register::Register::FILTER`] register.
    pub const fn val(self) -> u8 {
        self as u8
    }

    /// Output Data Rate in Hertz.
    pub const fn odr(self) -> f32 {
        match self {
            Odr::_4000Hz => 4000.0,
            Odr::_2000Hz => 2000.0,
            Odr::_1000Hz => 1000.0,
            Odr::_500Hz => 500.0,
            Odr::_250Hz => 250.0,
            Odr::_125Hz => 125.0,
            Odr::_62_5Hz => 62.5,
            Odr::_31_25Hz => 31.25,
            Odr::_15_625Hz => 15.625,
            Odr::_7_813Hz => 7.813,
            Odr::_3_906Hz => 3.906,
        }
    }

    /// Low-pass filter corner in Hertz.
    pub const fn lpf(self) -> f32 {
        match self {
            Odr::_4000Hz => 1000.0,
            Odr::_2000Hz => 500.0,
            Odr::_1000Hz => 250.0,
            Odr::_500Hz => 125.0,
            Odr::_250Hz => 62.5,
            Odr::_125Hz => 31.25,
            Odr::_62_5Hz => 15.625,
            Odr::_31_25Hz => 7.813,
            Odr::_15_625Hz => 3.906,
            Odr::_7_813Hz => 1.953,
            Odr::_3_906Hz => 0.977,
        }
    }
}

/// Enables retrieving ODR_LPF enum value from the [`crate::register::Register::FILTER`] register value.
impl From<u8> for Odr {
    fn from(value: u8) -> Self {
        match value & 0x0F {
            0 => Odr::_4000Hz,
            1 => Odr::_2000Hz,
            2 => Odr::_1000Hz,
            3 => Odr::_500Hz,
            4 => Odr::_250Hz,
            5 => Odr::_125Hz,
            6 => Odr::_62_5Hz,
            7 => Odr::_31_25Hz,
            8 => Odr::_15_625Hz,
            9 => Odr::_7_813Hz,
            10 => Odr::_3_906Hz,
            unexpected => panic!("Unexpected value {} for ODR_LPFs", unexpected),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Accelerometer's −3 dB filter corner for the first-order.
/// The ADXL355 offers 7 different configurations, the high-pass filter frequency is relative to the ODR.
pub enum HpfCorner {
    /// Not applicable, no high-pass filter enabled
    #[default]
    Off = 0,
    /// 24.7 × 10^-4 × ODR
    _24_7 = 1,
    /// 6.2048 × 10^-4 × ODR
    _6_2084 = 2,
    /// 1.5454 × 10^-4 × ODR
    _1_5545 = 3,
    /// 0.3862 × 10^-4 × ODR
    _0_3862 = 4,
    /// 0.0954 × 10^-4 × ODR
    _0_0954 = 5,
    /// 0.0238 × 10^-4 × ODR
    _0_0238 = 6,
}

impl HpfCorner {
    /// HPF_CORNER configuration value as expected in the HPF_CORNER bits of the [`crate::register::Register::FILTER`] register.
    pub const fn val(self) -> u8 {
        self as u8
    }

    /// Coefficient of the high-pass filter corner relative to the output data rate.
    /// To get the high-pass filter corner in Hz, multiply this coefficient to the odr value.
    pub const fn hpf_coefficient(self) -> Option<f32> {
        match self {
            HpfCorner::Off => None,
            HpfCorner::_24_7 => Some(24.7e-4),
            HpfCorner::_6_2084 => Some(6.2084e-4),
            HpfCorner::_1_5545 => Some(1.5545e-4),
            HpfCorner::_0_3862 => Some(0.3862e-4),
            HpfCorner::_0_0954 => Some(0.0954e-4),
            HpfCorner::_0_0238 => Some(0.0238e-4),
        }
    }
}

/// Enables retrieving HPF_CORNER enum value from the [`crate::register::Register::FILTER`] register value.
impl From<u8> for HpfCorner {
    fn from(value: u8) -> Self {
        match (value & 0x70) >> 4 {
            0 => HpfCorner::Off,
            1 => HpfCorner::_24_7,
            2 => HpfCorner::_6_2084,
            3 => HpfCorner::_1_5545,
            4 => HpfCorner::_0_3862,
            5 => HpfCorner::_0_0954,
            6 => HpfCorner::_0_0238,
            unexpected => panic!("Unexpected value {} for HPF_CORNER", unexpected),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Configuration for the ADXL355.
/// The default configuration [`Self::default()`] uses:
///
/// - 2G for the range
/// - 3.906Hz for the output data rate
/// - No high pass filter
pub struct Config {
    pub range: Range,
    pub odr: Odr,
    pub hpf: HpfCorner,
}

impl Config {
    /// Creates a new configuration with provided values.
    pub fn new(range: Range, odr: Odr, hpf: HpfCorner) -> Self {
        Config { range, odr, hpf }
    }

    /// Sets the range.
    pub fn range(&mut self, range: Range) -> &mut Self {
        self.range = range;
        self
    }

    /// Sets the output data rate and low pass filter frequency.
    pub fn odr(&mut self, odr: Odr) -> &mut Self {
        self.odr = odr;
        self
    }

    /// Sets the -3dB corner frequency for the high pass filter
    pub fn hpf(&mut self, hpf: HpfCorner) -> &mut Self {
        self.hpf = hpf;
        self
    }
}
