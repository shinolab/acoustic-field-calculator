/*
 * File: power_field.rs
 * Project: field_buffer
 * Created Date: 20/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::*;
use crate::core::{Complex, Float};

/// Power field
pub struct PowerField {}

impl FieldType for PowerField {
    type Output = Float;

    fn calc_from_complex_pressure(cp: Complex) -> Self::Output {
        cp.norm_sqr()
    }
}
