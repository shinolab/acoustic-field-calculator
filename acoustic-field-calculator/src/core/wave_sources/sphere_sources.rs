/*
 * File: sphere_sources.rs
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
use crate::core::{utils::calc_wavelength, Complex, Float, Vector3, PI};

use num::Zero;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
/// Wave source which emits a simple spherical wave
pub struct SphereWaveSource {
    pos: Vector3,
    amp: Float,
    phase: Float,
    frequency: Float,
    wavenumber: Float,
}

impl SphereWaveSource {
    /// Returns a SphereWaveSource
    ///
    /// # Arguments
    ///
    /// * `pos` - Position of the source
    /// * `amp` - Amplitude of the source
    /// * `phase` - Phase of the source
    /// * `frequency` - Frequency of the source
    pub fn new(pos: Vector3, amp: Float, phase: Float, frequency: Float) -> Self {
        Self {
            pos,
            amp,
            phase,
            frequency,
            wavenumber: 0.,
        }
    }
}

impl WaveSource for SphereWaveSource {
    fn propagate(&self, x: Vector3) -> Complex {
        let diff = self.pos - x;
        let dist = diff.norm();
        let phase = self.phase + self.wavenumber * dist;
        let r = self.amp / dist;
        Complex::new(r * phase.cos(), r * phase.sin())
    }
    fn set_sound_speed(&mut self, c: Float) {
        self.wavenumber = 2.0 * PI / calc_wavelength(self.frequency, c);
    }
    impl_getset!((get = wavenumber, field = wavenumber), Float);
    impl_getset!((get = position, set = set_position, field = pos), Vector3);
    impl_getset!((get = phase, set = set_phase, field = phase), Float);
    impl_getset!((get = amp, set = set_amp, field = amp), Float);
}

impl std::default::Default for SphereWaveSource {
    fn default() -> Self {
        Self::new(Vector3::zero(), 1., 0., 40e3)
    }
}
