/*
 * File: bessel.rs
 * Project: src
 * Created Date: 05/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::*;

/// Producing a bessel beam
/// * Hasegawa, Keisuke, et al. "Electronically steerable ultrasound-driven long narrow air stream." Applied Physics Letters 111.6 (2017): 064104.
pub struct BesselBeam {
    point: Vector3,
    dir: Vector3,
    theta: Float,
}

impl BesselBeam {
    pub fn new(point: Vector3, dir: Vector3, theta: Float) -> Self {
        Self { point, dir, theta }
    }
}

impl Optimizer for BesselBeam {
    fn optimize<S: WaveSource>(&self, system: &mut UniformSystem<S>) {
        let point = self.point;
        let dir = self.dir;
        let sound_speed = system.sound_speed();
        let v = Vector3::new(dir[1], -dir[0], 0.);
        let theta_w = v.norm().asin();
        for source in system.wave_sources_mut() {
            let pos = source.position();

            let r = pos - point;
            let xr = r.cross(&v);
            let r = r * theta_w.cos() + xr * theta_w.sin() + v * (v.dot(&r) * (1. - theta_w.cos()));
            let dist =
                self.theta.sin() * (r[0] * r[0] + r[1] * r[1]).sqrt() - self.theta.cos() * r[2];

            let wave_length = sound_speed / source.frequency();
            let phase = (dist % wave_length) / wave_length;
            let phase = -2.0 * PI * phase;
            source.set_phase(phase);
        }
    }
}
