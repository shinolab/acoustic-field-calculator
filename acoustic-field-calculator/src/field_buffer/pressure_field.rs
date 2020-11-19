/*
 * File: pressure_field.rs
 * Project: field_buffer
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::*;
use crate::core::{Complex, Float};

/// Pressure field
pub struct PressureField {
    buf: Vec<Float>,
}

impl PressureField {
    pub fn new() -> Self {
        Self { buf: vec![] }
    }
}

impl Default for PressureField {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldBuffer<Float> for PressureField {
    fn calc_from_complex_pressure(cp: Complex) -> Float {
        cp.norm_sqr().sqrt()
    }

    fn buffer(&self) -> &[Float] {
        &self.buf
    }

    fn buffer_mut(&mut self) -> &mut Vec<Float> {
        &mut self.buf
    }
}
