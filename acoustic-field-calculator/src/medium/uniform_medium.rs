/*
 * File: uniform_medium.rs
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

use super::traits::Medium;
use crate::core::{wave_sources::WaveSource, Complex, Float, Vector3};

pub struct UniformMedium {
    sound_speed: Float,
    atten_coef: Float,
}

impl Medium for UniformMedium {
    fn propagate<S: crate::wave_sources::WaveSource>(
        &self,
        source: &S,
        target: Vector3,
    ) -> Complex {
        let diff = crate::fmath::sub(target, source.position());
        let dist = diff.norm();
        let theta = crate::fmath::acos(source.direction().dot(&diff) / dist);
        let d = S::directivity(theta);
        let r = source.amp() * d / dist;
        let phi = source.phase() + source.wavenumber() * dist;
        Complex::new(r * phi.cos(), r * phi.sin())
    }
}
