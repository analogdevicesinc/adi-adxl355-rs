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
/// ADXL355 interrupt pins configuration.
pub struct InterruptConfig(pub u8);

impl InterruptConfig {
    /// No interrupts are enabled on pins INT1 and INT2.
    pub fn all_off() -> Self {
        Self(0)
    }

    /// All the interrupts are enabled on pins INT1 and INT2.
    pub fn all_on() -> Self {
        Self(0xFF)
    }

    /// Enable activity interupt on pin INT2
    pub fn with_activity_on_int2(&mut self) -> Self {
        self.0 |= 0x80;
        *self
    }

    /// Enable activity interupt on pin INT1
    pub fn with_activity_on_int1(&mut self) -> Self {
        self.0 |= 0x08;
        *self
    }

    /// Enable fifo overrun interupt on pin INT2
    pub fn with_fifo_overrun_on_int2(&mut self) -> Self {
        self.0 |= 0x40;
        *self
    }

    /// Enable fifo overrun interupt on pin INT1
    pub fn with_fifo_overrun_on_int1(&mut self) -> Self {
        self.0 |= 0x04;
        *self
    }

    /// Enable fifo full interupt on pin INT2
    pub fn with_fifo_full_on_int2(&mut self) -> Self {
        self.0 |= 0x20;
        *self
    }

    /// Enable fifo full interupt on pin INT1
    pub fn with_fifo_full_on_int1(&mut self) -> Self {
        self.0 |= 0x02;
        *self
    }

    /// Enable data ready interupt on pin INT2
    pub fn with_data_ready_on_int2(&mut self) -> Self {
        self.0 |= 0x10;
        *self
    }

    /// Enable data ready interupt on pin INT1
    pub fn with_data_ready_on_int1(&mut self) -> Self {
        self.0 |= 0x01;
        *self
    }
}

impl Default for InterruptConfig {
    /// No interrupts are enabled on pins INT1 and INT2.
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
/// ADXL355 interrupt pins polarity.
pub enum InterruptPolarity {
    ActiveLow = 0,
    ActiveHigh = 1,
}
