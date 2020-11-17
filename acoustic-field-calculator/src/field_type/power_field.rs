/*
 * File: power_field.rs
 * Project: field_buffer
 * Created Date: 20/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::*;
use crate::{
    core::{container::WaveSourceContainer, Complex, Float},
    observe_area::ObserveArea,
    wave_sources::*,
};

/// Power field
pub struct PowerField {}

impl FieldType for PowerField {
    type Output = Float;
}

impl FieldCalculable for PowerField {
    fn calculate_field<S: WaveSource, A: ObserveArea<PowerField>>(
        container: &mut WaveSourceContainer<S>,
        observe_area: &mut A,
    ) {
        let wave_sources = container.wave_sources();
        calc_from_complex_pressure!(wave_sources, S, observe_area, c, c.norm_sqr());
    }
}
