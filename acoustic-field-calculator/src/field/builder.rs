/*
 * File: builder.rs
 * Project: field
 * Created Date: 16/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::*;
use crate::core::Float;
use std::{marker::PhantomData, mem::transmute};

pub struct Pressure;
pub struct Power;
pub struct ComplexPressure;

pub struct Empty;
pub struct Filled;

pub struct FieldBuilder {}

impl FieldBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct UniformFieldBuilder<C, T> {
    sound_speed: Float,
    sound_speed_state: PhantomData<C>,
    calc_type: PhantomData<T>,
}

impl Default for FieldBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldBuilder {
    /// Set filed type to pressure
    pub fn pressure(self) -> UniformFieldBuilder<Empty, Pressure> {
        UniformFieldBuilder::new()
    }
    /// Set filed type to power
    pub fn power(self) -> UniformFieldBuilder<Empty, Power> {
        UniformFieldBuilder::new()
    }
    /// Set filed type to complex pressure
    pub fn complex_pressure(self) -> UniformFieldBuilder<Empty, ComplexPressure> {
        UniformFieldBuilder::new()
    }
}
impl<T> UniformFieldBuilder<Empty, T> {
    fn new() -> UniformFieldBuilder<Empty, T> {
        Self {
            sound_speed: Default::default(),
            sound_speed_state: PhantomData,
            calc_type: PhantomData,
        }
    }

    pub fn sound_speed(mut self, c: Float) -> UniformFieldBuilder<Filled, T> {
        self.sound_speed = c;
        unsafe { transmute(self) }
    }
}

impl UniformFieldBuilder<Filled, Pressure> {
    pub fn build(self) -> PressureField {
        PressureField::new(self.sound_speed)
    }
}
impl UniformFieldBuilder<Filled, Power> {
    pub fn build(self) -> PowerField {
        PowerField::new(self.sound_speed)
    }
}
impl UniformFieldBuilder<Filled, ComplexPressure> {
    pub fn build(self) -> ComplexPressureField {
        ComplexPressureField::new(self.sound_speed)
    }
}
