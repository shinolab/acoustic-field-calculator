/*
 * File: complex_pressure_field.rs
 * Project: field_buffer
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/10/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::*;
use crate::calculator::*;
use crate::core::Float;
use crate::na::ComplexField;
use crate::observe_area::*;
use crate::wave_sources::*;
use crate::Complex;

/// Pressure field
pub struct ComplexPressureField<T> {
    results: Vec<T>,
}

impl<T> ComplexPressureField<T> {
    pub fn new() -> Self {
        Self { results: vec![] }
    }
}

impl<T> Default for ComplexPressureField<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FieldBuffer<T> for ComplexPressureField<T> {
    fn buffer(&self) -> &[T] {
        &self.results
    }
    fn buffer_mut(&mut self) -> &mut Vec<T> {
        &mut self.results
    }
}

impl FieldBufferCalculable<Complex> for ComplexPressureField<Complex> {
    fn calculate_field<S, F>(&mut self, calculator: &CpuCalculator<S>, observe_area: &F)
    where
        S: WaveSource,
        F: ObserveArea,
    {
        let wave_sources = calculator.wave_sources();
        calc_from_complex_pressure!(wave_sources, observe_area, c, c, &mut self.results);
    }
}

impl ScalarFieldBuffer<Complex> for ComplexPressureField<Complex> {
    fn max(&self) -> Complex {
        self.buffer()
            .iter()
            .fold(Complex::new(Float::NAN, Float::NAN), |m, &v| {
                if v.abs() < m.abs() {
                    m
                } else {
                    v
                }
            })
    }
}
