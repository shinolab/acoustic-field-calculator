/*
 * File: power_field.rs
 * Project: field_buffer
 * Created Date: 20/09/2020
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

/// Power field
pub struct PowerField {
    buf: Vec<Float>,
}

impl PowerField {
    pub fn new() -> Self {
        Self { buf: vec![] }
    }
}

impl Default for PowerField {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldBuffer<Float> for PowerField {
    fn calc_from_complex_pressure(cp: Complex) -> Float {
        cp.norm_sqr()
    }

    fn buffer(&self) -> &[Float] {
        &self.buf
    }

    fn buffer_mut(&mut self) -> &mut Vec<Float> {
        &mut self.buf
    }
}
