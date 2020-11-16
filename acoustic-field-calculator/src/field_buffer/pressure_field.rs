/*
 * File: pressure_field.rs
 * Project: field_buffer
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2020
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

/// Pressure field
pub struct PressureField<T> {
    results: Vec<T>,
}

impl<T> PressureField<T> {
    pub fn new() -> Self {
        Self { results: vec![] }
    }
}

impl<T> Default for PressureField<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FieldBuffer<T> for PressureField<T> {
    fn buffer(&self) -> &[T] {
        &self.results
    }
    fn buffer_mut(&mut self) -> &mut Vec<T> {
        &mut self.results
    }
}

impl FieldBufferCalculable<Float> for PressureField<Float> {
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
            c.norm_sqr().sqrt(),
            &mut self.results
        );
    }
}

impl ScalarFieldBuffer<f32> for PressureField<f32> {
    fn max(&self) -> f32 {
        self.buffer().iter().fold(f32::NAN, |m, v| v.max(m))
    }
}

impl ScalarFieldBuffer<f64> for PressureField<f64> {
    fn max(&self) -> f64 {
        self.buffer().iter().fold(f64::NAN, |m, v| v.max(m))
    }
}
