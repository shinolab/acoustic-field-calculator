/*
 * File: complex_pressure_field.rs
 * Project: field_buffer
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::*;
use crate::{
    core::{container::WaveSourceContainer, Complex, Float},
    na::ComplexField,
    observe_area::*,
    wave_sources::*,
};

/// Pressure field
pub struct ComplexPressureField {
    results: Vec<Complex>,
    sound_speed: Float,
}

impl ComplexPressureField {
    pub(crate) fn new(c: Float) -> Self {
        Self {
            results: vec![],
            sound_speed: c,
        }
    }
}

impl LinearUniformField for ComplexPressureField {
    fn sound_speed(&self) -> Float {
        self.sound_speed
    }
}

impl FieldBuffer<Complex> for ComplexPressureField {
    fn buffer(&self) -> &[Complex] {
        &self.results
    }
    fn buffer_mut(&mut self) -> &mut Vec<Complex> {
        &mut self.results
    }
}

impl FieldCalculable<Complex> for ComplexPressureField {
    fn calculate_field<S, F>(&mut self, container: &mut WaveSourceContainer<S>, observe_area: &F)
    where
        S: WaveSource,
        F: ObserveArea,
    {
        for wave_source in container.wave_sources_mut() {
            wave_source.set_sound_speed(self.sound_speed);
        }
        let wave_sources = container.wave_sources();
        calc_from_complex_pressure!(wave_sources, observe_area, c, c, &mut self.results);
    }
}

impl ScalarFieldBuffer<Complex> for ComplexPressureField {
    fn max(&self) -> Complex {
        self.buffer().iter().fold(
            Complex::new(Float::NAN, Float::NAN),
            |m, &v| -> na::Complex<f32> {
                if v.abs() < m.abs() {
                    m
                } else {
                    v
                }
            },
        )
    }
}
