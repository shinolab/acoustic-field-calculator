/*
 * File: traits.rs
 * Project: buffer
 * Created Date: 08/05/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::core::Vector3;
use crate::field_type::FieldType;

pub trait ObserveArea<F: FieldType> {
    /// Returns all observation points
    fn points_and_results_buf(&mut self) -> (&Vec<Vector3>, &mut Vec<F::Output>);
    fn results(&self) -> &[F::Output];
}

pub trait ScalarFieldBuffer<D> {
    fn max_result(&self) -> D;
}
