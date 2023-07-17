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
/// ADXL355 status providing its various conditions.
pub struct Status(pub u8);

impl Status {
    pub const NVM_BUSY_BIT: u8 = 0x10;
    pub const ACTIVITY_BIT: u8 = 0x08;
    pub const FIFO_OVERRUN_BIT: u8 = 0x04;
    pub const FIFO_FULL_BIT: u8 = 0x02;
    pub const DATA_READY_BIT: u8 = 0x01;

    /// NVM controller is busy with a refresh, programming, or a built in self test (BIST).
    pub fn is_nvm_busy(&self) -> bool {
        self.0 & Self::NVM_BUSY_BIT == Self::NVM_BUSY_BIT
    }

    /// Activity, as defined in the ACT_THRESH_x and ACT_COUNT registers, is detected.
    pub fn is_activity_detected(&self) -> bool {
        self.0 & Self::ACTIVITY_BIT == Self::ACTIVITY_BIT
    }

    /// FIFO has overrun, and the oldest data is lost.
    pub fn is_fifo_overrun(&self) -> bool {
        self.0 & Self::FIFO_OVERRUN_BIT == Self::FIFO_OVERRUN_BIT
    }

    /// FIFO watermark is reached.
    pub fn is_fifo_full(&self) -> bool {
        self.0 & Self::FIFO_FULL_BIT == Self::FIFO_FULL_BIT
    }

    /// A complete x-axis, y-axis, and z-axis measurement was made and results can be read.
    pub fn is_data_ready(&self) -> bool {
        self.0 & Self::DATA_READY_BIT == Self::DATA_READY_BIT
    }
}
