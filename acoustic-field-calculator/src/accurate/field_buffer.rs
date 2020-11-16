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
    field::*,
    observe_area::*,
    prelude::FieldBuffer,
    wave_sources::*,
};

impl AccurateFieldBuffer<Float> for PressureField {
    fn calculate_field<S, F>(&mut self, container: &mut WaveSourceContainer<S>, observe_area: &F)
    where
        S: WaveSource,
        F: ObserveArea,
    {
        for wave_source in container.wave_sources_mut() {
            wave_source.set_sound_speed(self.sound_speed());
        }
        let wave_sources = container.wave_sources();
        calc_from_complex_pressure_accurate!(
            wave_sources,
            observe_area,
            c,
            c.norm_sqr().sqrt(),
            self.buffer_mut()
        );
    }
}

impl AccurateFieldBuffer<Complex> for ComplexPressureField {
    fn calculate_field<S, F>(&mut self, container: &mut WaveSourceContainer<S>, observe_area: &F)
    where
        S: WaveSource,
        F: ObserveArea,
    {
        for wave_source in container.wave_sources_mut() {
            wave_source.set_sound_speed(self.sound_speed());
        }
        let wave_sources = container.wave_sources();
        calc_from_complex_pressure_accurate!(wave_sources, observe_area, c, c, self.buffer_mut());
    }
}

impl AccurateFieldBuffer<Float> for PowerField {
    fn calculate_field<S, F>(&mut self, container: &mut WaveSourceContainer<S>, observe_area: &F)
    where
        S: WaveSource,
        F: ObserveArea,
    {
        for wave_source in container.wave_sources_mut() {
            wave_source.set_sound_speed(self.sound_speed());
        }
        let wave_sources = container.wave_sources();
        calc_from_complex_pressure_accurate!(
            wave_sources,
            observe_area,
            c,
            c.norm_sqr(),
            self.buffer_mut()
        );
    }
}
