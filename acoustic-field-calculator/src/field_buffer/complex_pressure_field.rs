/*
 * File: complex_pressure_field.rs
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
use crate::core::Complex;

/// Complex pressure field
pub struct ComplexPressureField {
    buf: Vec<Complex>,
}

impl ComplexPressureField {
    pub fn new() -> Self {
        Self { buf: vec![] }
    }
}

impl Default for ComplexPressureField {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldBuffer<Complex> for ComplexPressureField {
    fn calc_from_complex_pressure(cp: Complex) -> Complex {
        cp
    }

    fn buffer(&self) -> &[Complex] {
        &self.buf
    }

    fn buffer_mut(&mut self) -> &mut Vec<Complex> {
        &mut self.buf
    }
}
