/*
 * File: builder.rs
 * Project: calculator
 * Created Date: 19/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::*;
use crate::wave_sources::*;

use crate::*;
use std::marker::PhantomData;
use std::mem::transmute;

pub struct Normal;

pub struct Empty;
pub struct Filled;

/// Builder for calculators
pub struct CalculatorBuilder<C, T> {
    sound_speed: Float,
    sound_speed_state: PhantomData<C>,
    calc_type: PhantomData<T>,
}

impl CalculatorBuilder<Empty, Normal> {
    pub fn new() -> Self {
        Self {
            sound_speed: Default::default(),
            sound_speed_state: PhantomData,
            calc_type: PhantomData,
        }
    }
}

impl<T> CalculatorBuilder<Empty, T> {
    /// Set speed of sound
    ///
    /// # Arguments
    ///
    /// * `c` - speed of sound
    pub fn set_sound_speed(mut self, c: Float) -> CalculatorBuilder<Filled, T> {
        self.sound_speed = c;
        unsafe { transmute(self) }
    }
}

impl CalculatorBuilder<Filled, Normal> {
    /// Generate normal calculator
    /// You must specify speed of sound by [set_sound_speed](#method.set_sound_speed)
    pub fn generate<S: WaveSource>(self) -> CpuCalculator<S> {
        CpuCalculator::new(self.sound_speed)
    }
}

impl<T> CalculatorBuilder<Filled, T> {
    pub fn sound_speed(&self) -> Float {
        self.sound_speed
    }
}

impl Default for CalculatorBuilder<Empty, Normal> {
    fn default() -> Self {
        Self::new()
    }
}
