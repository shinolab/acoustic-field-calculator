/*
 * File: focal_point.rs
 * Project: multiple_foci
 * Created Date: 27/05/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::PI;

use crate::Optimizer;
use crate::Vector3;
use crate::WaveSource;

pub struct FocalPoint {
    point: Vector3,
}

impl FocalPoint {
    pub fn new(point: Vector3) -> Self {
        Self { point }
    }
}

impl Optimizer for FocalPoint {
    fn optimize<S: WaveSource>(&self, wave_sources: &mut [S]) {
        let focal_point = self.point;
        for source in wave_sources {
            let pos = source.position();

            let d = (pos - focal_point).norm();

            let wave_length = 2.0 * PI / source.wavenumber();
            let phase = (d % wave_length) / wave_length;
            let phase = -2.0 * PI * phase;
            source.set_phase(phase);
        }
    }
}