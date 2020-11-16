/*
 * File: pressure_field.rs
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
    observe_area::*,
    wave_sources::*,
};

/// Pressure field
pub struct PressureField {
    results: Vec<Float>,
    sound_speed: Float,
}

impl PressureField {
    pub(crate) fn new(c: Float) -> Self {
        Self {
            results: vec![],
            sound_speed: c,
        }
    }
}

impl LinearUniformField for PressureField {
    fn sound_speed(&self) -> Float {
        self.sound_speed
    }
}

impl FieldBuffer<Float> for PressureField {
    fn buffer(&self) -> &[Float] {
        &self.results
    }
    fn buffer_mut(&mut self) -> &mut Vec<Float> {
        &mut self.results
    }
}

impl FieldCalculable<Float> for PressureField {
    fn calculate_field<S, F>(&mut self, container: &mut WaveSourceContainer<S>, observe_area: &F)
    where
        S: WaveSource,
        F: ObserveArea,
    {
        for wave_source in container.wave_sources_mut() {
            wave_source.set_sound_speed(self.sound_speed);
        }
        let wave_sources = container.wave_sources();
        calc_from_complex_pressure!(
            wave_sources,
            observe_area,
            c,
            c.norm_sqr().sqrt(),
            &mut self.results
        );
    }
}

impl ScalarFieldBuffer<Float> for PressureField {
    fn max(&self) -> Float {
        self.buffer().iter().fold(Float::NAN, |m, v| v.max(m))
    }
}
