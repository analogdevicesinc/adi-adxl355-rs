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

#[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
/// ADXL355 register map.
/// See [datasheet](https://www.analog.com/media/en/technical-documentation/data-sheets/adxl354_adxl355.pdf).
pub enum Register {
    /// This register contains the Analog Devices ID, 0xAD.
    DEVID_AD = 0x00,
    /// This register contains the Analog Devices MEMS ID, 0x1D.
    DEVID_MST = 0x01,
    /// This register contains the device ID, 0xED (355 octal).
    PARTID = 0x02,
    /// This register contains the product revision ID, beginning with 0x00 and incrementing for each subsequent revision.
    REVID = 0x03,
    /// This register includes bits that describe the various conditions of the ADXL355.
    STATUS = 0x04,
    /// This register indicates the number of valid data samples present in the FIFO buffer. This number ranges from 0 to 96.
    FIFO_ENTRIES = 0x05,
    /// [`Register::TEMP2`] and [`Register::TEMP1`] contain the uncalibrated temperature data.
    /// The nominal intercept is 1885 LSB at 25°C and the nominal slope is −9.05 LSB/°C.
    /// TEMP2 contains the four most significant bits of the uncalibrated temperature data (12-bit value).
    TEMP2 = 0x06,
    /// TEMP1 contains the eight least significant bits of the uncalibrated temperature data (12-bit value).
    /// The ADXL355 temperature value is not double buffered, meaning the value can update between reading of the two registers.
    TEMP1 = 0x07,
    /// [`Register::XDATA3`], [`Register::XDATA2`], and [`Register::XDATA1`] contain the x-axis acceleration data. Data is left justified and formatted as twos complement.
    /// XDATA, Bits\[19:12\]
    XDATA3 = 0x08,
    /// XDATA, Bits\[11:4\]
    XDATA2 = 0x09,
    /// XDATA, Bits\[3:0\]
    XDATA1 = 0x0A,
    /// [`Register::YDATA3`], [`Register::YDATA2`], and [`Register::YDATA1`] contain the y-axis acceleration data. Data is left justified and formatted as twos complement.
    /// YDATA, Bits\[19:12\]
    YDATA3 = 0x0B,
    /// YDATA, Bits\[11:4\]
    YDATA2 = 0x0C,
    /// YDATA, Bits\[3:0\]
    YDATA1 = 0x0D,
    /// [`Register::ZDATA3`], [`Register::ZDATA2`], and [`Register::ZDATA1`] contain the z-axis acceleration data. Data is left justified and formatted as twos complement.
    /// ZDATA, Bits\[19:12\]
    ZDATA3 = 0x0E,
    /// ZDATA, Bits\[11:4\]
    ZDATA2 = 0x0F,
    /// ZDATA, Bits\[3:0\]
    ZDATA1 = 0x10,
    /// Read this register to access data stored in the FIFO.
    FIFO_DATA = 0x11,
    /// [`Register::OFFSET_X_H`], and [`Register::OFFSET_X_L`] contain the offset added to x-axis data after all other signal processing.
    /// OFFSET_X, Bits\[15:8\]
    OFFSET_X_H = 0x1E,
    /// OFFSET_X, Bits\[7:0\]
    OFFSET_X_L = 0x1F,
    /// [`Register::OFFSET_Y_H`], and [`Register::OFFSET_Y_L`] contain the offset added to y-axis data after all other signal processing.
    /// OFFSET_Y, Bits\[15:8\]
    OFFSET_Y_H = 0x20,
    /// OFFSET_Y, Bits\[7:0\]
    OFFSET_Y_L = 0x21,
    /// [`Register::OFFSET_Z_H`], and [`Register::OFFSET_Z_L`] contain the offset added to z-axis data after all other signal processing.
    /// OFFSET_Z, Bits\[15:8\]
    OFFSET_Z_H = 0x22,
    /// OFFSET_Z, Bits\[7:0\]
    OFFSET_Z_L = 0x23,
    /// Activity enable register.
    ACT_EN = 0x24,
    /// [`Register::ACT_THRESH_H`], and [`Register::ACT_THRESH_L`] contain the threshold for activity detection.
    /// ACT_THRESH, Bits\[15:8\]
    ACT_THRESH_H = 0x25,
    /// ACT_THRESH, Bits\[7:0\]
    ACT_THRESH_L = 0x26,
    /// Number of consecutive events above threshold (from ACT_THRESH) required to detect activity.
    ACT_COUNT = 0x27,
    /// Use this register to specify parameters for the internal high-pass and low-pass filters.
    FILTER = 0x28,
    /// Use the FIFO_SAMPLES value to specify the number of samples to store in the FIFO.
    /// The default value of this register is 0x60 to avoid triggering the FIFO watermark interrupt.
    FIFO_SAMPLES = 0x29,
    /// The INT_MAP register configures the interrupt pins.
    /// Bits\[7:0\] select which functions generate an interrupt on the INT1 and INT2 pins.
    /// Multiple events can be configured.
    /// If the corresponding bit is set to 1, the function generates an interrupt on the interrupt pins.
    INT_MAP = 0x2A,
    /// Use this register to control the external timing triggers.
    SYNC = 0x2B,
    /// I2C speed, interrupt polarity, and range.
    RANGE = 0x2C,
    /// Power control.
    POWER_CTL = 0x2D,
    /// Self test feature.
    SELF_TEST = 0x2E,
    /// Write Code 0x52 in this register to reset the device.
    RESET = 0x2F,
    /// Shadow registers used to check if software reset is successful.
    /// Shadow register 1.
    SHADOW_REG1 = 0x50,
    /// Shadow register 2.
    SHADOW_REG2 = 0x51,
    /// Shadow register 3.
    SHADOW_REG3 = 0x52,
    /// Shadow register 4.
    SHADOW_REG4 = 0x53,
    /// Shadow register 5.
    SHADOW_REG5 = 0x54,
}

impl Register {
    /// Get register address
    pub fn addr(self) -> u8 {
        self as u8
    }

    /// Check if the register address is read-only.
    pub fn read_only(self) -> bool {
        matches!(
            self,
            Register::DEVID_AD
                | Register::DEVID_MST
                | Register::PARTID
                | Register::REVID
                | Register::STATUS
                | Register::FIFO_ENTRIES
                | Register::TEMP2
                | Register::TEMP1
                | Register::XDATA3
                | Register::XDATA2
                | Register::XDATA1
                | Register::YDATA3
                | Register::YDATA2
                | Register::YDATA1
                | Register::ZDATA3
                | Register::ZDATA2
                | Register::ZDATA1
                | Register::FIFO_DATA
                | Register::SHADOW_REG1
                | Register::SHADOW_REG2
                | Register::SHADOW_REG3
                | Register::SHADOW_REG4
                | Register::SHADOW_REG5
        )
    }
}
