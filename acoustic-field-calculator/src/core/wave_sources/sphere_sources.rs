/*
 * File: sphere_sources.rs
 * Project: core
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::*;
use crate::core::{Float, Vector3};

use num::Zero;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
/// Wave source which emits a simple spherical wave
pub struct SphereWaveSource {
    pos: Vector3,
    amp: Float,
    phase: Float,
    frequency: Float,
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
        }
    }
}

impl WaveSource for SphereWaveSource {
    #[inline(always)]
    fn directivity(_theta: Float) -> Float {
        1.0
    }
    impl_getset!((get = frequency, field = frequency), Float);
    impl_getset!((get = position, set = set_position, field = pos), Vector3);
    impl_getset!((get = phase, set = set_phase, field = phase), Float);
    impl_getset!((get = amp, set = set_amp, field = amp), Float);

    fn direction(&self) -> Vector3 {
        Vector3::zero()
    }
    fn set_direction(&mut self, _value: Vector3) {}
}
