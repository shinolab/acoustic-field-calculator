/*
 * File: uniform_medium.rs
 * Project: system
 * Created Date: 18/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{core::wave_sources::WaveSource, system::UniformSystem};

use super::traits::GpuPropagationMedium;

pub const GPU_DIRECTIVITY_CACHE_SIZE: usize = 1800;

impl<S: WaveSource> GpuPropagationMedium<S> for UniformSystem<S> {
    fn wavenums(&self) -> &[f32] {
        self.wavenums()
    }

    fn attenuations(&self) -> &[f32] {
        self.attens()
    }

    fn directivities(&self) -> Vec<f32> {
        let mut directivities = Vec::with_capacity(GPU_DIRECTIVITY_CACHE_SIZE);
        for i in 0..GPU_DIRECTIVITY_CACHE_SIZE {
            let theta = std::f32::consts::PI * (i as f32 / GPU_DIRECTIVITY_CACHE_SIZE as f32);
            let d = S::directivity(theta);
            directivities.push(d);
        }
        directivities
    }
}
