/*
 * File: traits.rs
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

use crate::core::wave_sources::WaveSource;
use crate::core::Float;
use crate::Vector3;

/// Gpu calculator requires directivity and attenuation coefficient.
pub trait GpuWaveSource: WaveSource {
    fn directivity() -> Vec<Float>;
    getset!((getter = direction), Vector3);
    getset!((getter = attenuation), Float);
}
