/*
 * File: complex_pressure_field.rs
 * Project: field_buffer
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::*;
use crate::core::Complex;

/// Pressure field
pub struct ComplexPressureField {}

impl FieldType for ComplexPressureField {
    type Output = Complex;

    fn calc_from_complex_pressure(cp: Complex) -> Self::Output {
        cp
    }
}
