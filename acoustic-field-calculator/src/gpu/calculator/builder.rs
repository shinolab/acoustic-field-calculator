/*
 * File: builder.rs
 * Project: calculator
 * Created Date: 19/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::*;
use crate::calculator::builder::{CalculatorBuilder, Filled, Normal};
use crate::gpu::*;
use std::mem::transmute;

pub struct GPU;

impl<C> CalculatorBuilder<C, Normal> {
    /// Set GPU mode
    pub fn gpu_enable(self) -> CalculatorBuilder<C, GPU> {
        unsafe { transmute(self) }
    }
}

impl CalculatorBuilder<Filled, GPU> {
    /// Generate Accurate calculator
    /// You must specify speed of sound by [set_sound_speed](#method.set_sound_speed)
    pub fn generate<S: GpuWaveSource>(self) -> GpuCalculator<S> {
        GpuCalculator::new(self.sound_speed())
    }
}
