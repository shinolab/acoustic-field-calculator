/*
 * File: traits.rs
 * Project: buffer
 * Created Date: 08/05/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::Vector3;

pub trait ObserveArea {
    /// Returns all observation points
    fn observe_points(&self) -> &Vec<Vector3>;
}
