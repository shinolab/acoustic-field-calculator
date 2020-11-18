/*
 * File: focal_point.rs
 * Project: multiple_foci
 * Created Date: 27/05/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::*;

pub struct FocalPoint {
    point: Vector3,
}

impl FocalPoint {
    pub fn new(point: Vector3) -> Self {
        Self { point }
    }
}

impl Optimizer for FocalPoint {
    fn optimize<S: WaveSource>(&self, system: &mut UniformSystem<S>) {
        let focal_point = self.point;
        let sound_speed = system.sound_speed();
        for source in system.wave_sources_mut() {
            let pos = source.position();
            let d = (pos - focal_point).norm();
            let wave_length = sound_speed / source.frequency();
            let phase = (d % wave_length) / wave_length;
            let phase = -2.0 * PI * phase;
            source.set_phase(phase);
        }
    }
}
