/*
 * File: traits.rs
 * Project: medium
 * Created Date: 18/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::core::wave_sources::WaveSource;
use crate::core::{Complex, Vector3};

pub trait Medium {
    fn propagate<S: WaveSource>(&self, source: &S, target: Vector3) -> Complex;
}
