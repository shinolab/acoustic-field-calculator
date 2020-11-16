/*
 * File: t4010a1.rs
 * Project: core
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::*;
use crate::core::{wave_sources::T4010A1, Float, Vector3};

#[cfg(feature = "gpu")]
impl GpuWaveSource for T4010A1 {
    fn directivity() -> Vec<Float> {
        T4010A1::directivity()
    }

    impl_getset!((get = direction, field = dir), Vector3);
    impl_getset!((get = attenuation, field = atten_coef), Float);
}
