/*
* File: traits.rs
* Project: system
* Created Date: 18/11/2020
* Author: Shun Suzuki
* -----
* Last Modified: 19/11/2020
* Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
* -----
* Copyright (c) 2020 Hapis Lab. All rights reserved.
*
*/

use crate::{core::wave_sources::WaveSource, system::WaveSourceContainer};

pub trait GpuPropagationMedium<S: WaveSource>: WaveSourceContainer<S> {
    fn wavenums(&self) -> &[f32];
    fn attenuations(&self) -> &[f32];
    fn directivities(&self) -> Vec<f32>;
}
