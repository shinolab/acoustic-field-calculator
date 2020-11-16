/*
 * File: pressure_field.rs
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

use super::calculator_acc::AccurateCalculator;
use super::traits::*;
use crate::core::Float;
use crate::field_buffer::*;
use crate::wave_sources::*;
use crate::Complex;
use crate::{calculator::WaveSourceContainer, field_buffer::PowerField};
use crate::{observe_area::*, prelude::FieldBuffer};

impl AccurateFieldBuffer<Float> for PressureField<Float> {
    fn calculate_field<S, F>(&mut self, calculator: &AccurateCalculator<S>, observe_area: &F)
    where
        S: WaveSource,
        F: ObserveArea,
    {
        let wave_sources = calculator.wave_sources();
        calc_from_complex_pressure_accurate!(
            wave_sources,
            observe_area,
            c,
            c.norm_sqr().sqrt(),
            self.buffer_mut()
        );
    }
}

impl AccurateFieldBuffer<Complex> for ComplexPressureField<Complex> {
    fn calculate_field<S, F>(&mut self, calculator: &AccurateCalculator<S>, observe_area: &F)
    where
        S: WaveSource,
        F: ObserveArea,
    {
        let wave_sources = calculator.wave_sources();
        calc_from_complex_pressure_accurate!(wave_sources, observe_area, c, c, self.buffer_mut());
    }
}

impl AccurateFieldBuffer<Float> for PowerField<Float> {
    fn calculate_field<S, F>(&mut self, calculator: &AccurateCalculator<S>, observe_area: &F)
    where
        S: WaveSource,
        F: ObserveArea,
    {
        let wave_sources = calculator.wave_sources();
        calc_from_complex_pressure_accurate!(
            wave_sources,
            observe_area,
            c,
            c.norm_sqr(),
            self.buffer_mut()
        );
    }
}
