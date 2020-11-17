/*
 * File: t4010a1.rs
 * Project: core
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::*;
use crate::core::{attenuation::attenuation_coef, utils::calc_wavelength, Float, Vector3, PI};

use num::Zero;

#[allow(clippy::excessive_precision, clippy::unreadable_literal)]
static DIR_COEF_A: [Float; 10] = [
    0.,
    1.0,
    1.0,
    1.0,
    0.891250938,
    0.707945784,
    0.501187234,
    0.354813389,
    0.251188643,
    0.199526231,
];

#[allow(clippy::excessive_precision, clippy::unreadable_literal)]
static DIR_COEF_B: [Float; 10] = [
    0.,
    0.,
    0.,
    -0.2633589359691162,
    -0.8910683499820542,
    -1.1924098538381465,
    -1.0439934287916925,
    -0.7015151838490918,
    -0.4471050803642494,
    -0.179254124486545,
];
#[allow(clippy::excessive_precision, clippy::unreadable_literal)]
static DIR_COEF_C: [Float; 10] = [
    0.,
    0.,
    0.,
    -2.586746661956269,
    -1.0097633564074366,
    -0.7167962799004945,
    0.027352097655185065,
    0.39509604454751995,
    1.0625664749725618,
    0.008242027161100222,
];
#[allow(clippy::excessive_precision, clippy::unreadable_literal)]
static DIR_COEF_D: [Float; 10] = [
    0.,
    0.,
    0.,
    0.0009174516976956062,
    0.0001704413361662757,
    4.362034715924911,
    -2.238476800444034,
    1.274774620435337,
    -1.1276955684895724,
    -0.9016569137759131,
];

const ANGLE_DIV: Float = PI / 18.0;

#[derive(Clone, Copy)]
#[repr(C)]
/// Ultrasound transducer [T4010A1](https://www.nicera.co.jp/products/ultrasonic-sensor/open-aperture-type) model
pub struct T4010A1 {
    pub pos: Vector3,
    pub dir: Vector3,
    pub amp: Float,
    pub phase: Float,
    pub frequency: Float,
    pub atten_coef: Float,
    wavenumber: Float,
}

impl T4010A1 {
    /// Returns a T4010A1 transducer
    ///
    /// # Arguments
    ///
    /// * `pos` - Position of the source
    /// * `dir` - Direction of the source
    /// * `amp` - Amplitude of the source
    /// * `phase` - Phase of the source
    /// * `frequency` - Frequency of the source
    pub fn new(pos: Vector3, dir: Vector3, amp: Float, phase: Float, frequency: Float) -> Self {
        Self {
            pos,
            dir,
            amp,
            phase,
            frequency,
            wavenumber: 0.,
            atten_coef: attenuation_coef(frequency, 30., 1., 1., 293.15, 293.15, 273.16),
        }
    }

    /// Set an attenuation coefficient
    ///
    /// # Arguments
    ///
    /// * `atten` - Attenuation coefficient
    pub fn set_attenuation(&mut self, atten: Float) {
        self.atten_coef = atten;
    }
}

impl WaveSource for T4010A1 {
    fn directivity(theta: Float) -> Float {
        match (theta / ANGLE_DIV).ceil() as usize {
            0 => 1.0,
            i if i > 9 => 0.0,
            i => {
                let a = DIR_COEF_A[i];
                let b = DIR_COEF_B[i];
                let c = DIR_COEF_C[i];
                let d = DIR_COEF_D[i];
                let x = theta - (i as Float - 1.0) * ANGLE_DIV;
                let res = a + (b + ((c + d * x) * x)) * x;
                res
            }
        }
    }

    fn set_sound_speed(&mut self, c: Float) {
        self.wavenumber = 2.0 * PI / calc_wavelength(self.frequency, c);
    }

    impl_getset!((get = wavenumber, field = wavenumber), Float);
    impl_getset!((get = position, set = set_position, field = pos), Vector3);
    impl_getset!((get = phase, set = set_phase, field = phase), Float);
    impl_getset!((get = amp, set = set_amp, field = amp), Float);
    impl_getset!((get = direction, set = set_direction, field = dir), Vector3);
}

impl std::default::Default for T4010A1 {
    fn default() -> Self {
        Self::new(Vector3::zero(), Vector3::z(), 1., 0., 40e3)
    }
}
