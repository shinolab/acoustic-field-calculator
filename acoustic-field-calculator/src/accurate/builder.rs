/*
 * File: builder.rs
 * Project: calculator
 * Created Date: 19/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::*;
use crate::calculator::builder::{CalculatorBuilder, Filled, Normal};
use crate::wave_sources::*;

use std::mem::transmute;

pub struct Accurate;

impl<C> CalculatorBuilder<C, Normal> {
    /// Enable accurate mode
    pub fn set_accurate(self) -> CalculatorBuilder<C, Accurate> {
        unsafe { transmute(self) }
    }
}

impl CalculatorBuilder<Filled, Accurate> {
    /// Generate Accurate calculator
    /// You must specify speed of sound by [set_sound_speed](#method.set_sound_speed)
    pub fn generate<S: WaveSource>(self) -> AccurateCalculator<S> {
        AccurateCalculator::new(self.sound_speed())
    }
}
