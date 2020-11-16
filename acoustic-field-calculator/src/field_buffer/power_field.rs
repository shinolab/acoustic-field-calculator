/*
 * File: power_field.rs
 * Project: field_buffer
 * Created Date: 20/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/10/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::*;
use crate::calculator::*;
use crate::core::Float;
use crate::observe_area::*;
use crate::wave_sources::*;
use crate::Complex;

/// Power field
pub struct PowerField<T> {
    results: Vec<T>,
}

impl<T> PowerField<T> {
    pub fn new() -> Self {
        Self { results: vec![] }
    }
}

impl<T> Default for PowerField<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FieldBuffer<T> for PowerField<T> {
    fn buffer(&self) -> &[T] {
        &self.results
    }
    fn buffer_mut(&mut self) -> &mut Vec<T> {
        &mut self.results
    }
}

impl FieldBufferCalculable<Float> for PowerField<Float> {
    fn calculate_field<S, F>(&mut self, calculator: &CpuCalculator<S>, observe_area: &F)
    where
        S: WaveSource,
        F: ObserveArea,
    {
        let wave_sources = calculator.wave_sources();
        calc_from_complex_pressure!(
            wave_sources,
            observe_area,
            c,
            c.norm_sqr(),
            &mut self.results
        );
    }
}

impl ScalarFieldBuffer<f32> for PowerField<f32> {
    fn max(&self) -> f32 {
        self.buffer().iter().fold(f32::NAN, |m, v| v.max(m))
    }
}

impl ScalarFieldBuffer<f64> for PowerField<f64> {
    fn max(&self) -> f64 {
        self.buffer().iter().fold(f64::NAN, |m, v| v.max(m))
    }
}
