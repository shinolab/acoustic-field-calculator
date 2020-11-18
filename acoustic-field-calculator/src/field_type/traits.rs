/*
* File: traits.rs
* Project: field_type
* Created Date: 18/09/2020
* Author: Shun Suzuki
* -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::core::Complex;

pub trait FieldType {
    type Output;
    fn calc_from_complex_pressure(cp: Complex) -> Self::Output;
}
