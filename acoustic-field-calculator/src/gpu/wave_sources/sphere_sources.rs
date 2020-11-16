/*
 * File: sphere_sources.rs
 * Project: core
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::*;
use crate::core::wave_sources::SphereWaveSource;
use crate::core::{Float, Vector3};

impl GpuWaveSource for SphereWaveSource {
    fn directivity() -> Vec<Float> {
        vec![1.0]
    }

    fn direction(&self) -> Vector3 {
        Default::default()
    }

    fn attenuation(&self) -> Float {
        0.
    }
}
