/*
 * File: t4010a1.rs
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
use crate::core::{
    attenuation::attenuation_coef, utils::calc_wavelength, Complex, Float, Vector3, PI,
};

use num::Zero;

const DIRECTIVITY_CACHE_SIZE: usize = 900;
const DIRECTIVITY_SLACE: Float = (DIRECTIVITY_CACHE_SIZE - 1) as Float / PI;

lazy_static! {
    static ref DIRECTIVITY: [Float; DIRECTIVITY_CACHE_SIZE] = {
        #[allow(clippy::excessive_precision, clippy::unreadable_literal)]
        static DIR_COEF_A: &[Float] = &[
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
        static DIR_COEF_B: &[Float] = &[
            0.,
            0.,
            0.,
            -0.00459648054721,
            -0.0155520765675,
            -0.0208114779827,
            -0.0182211227016,
            -0.0122437497109,
            -0.00780345575475,
            -0.00312857467007,
        ];

        #[allow(clippy::excessive_precision, clippy::unreadable_literal)]
        static DIR_COEF_C: &[Float] = &[
            0.,
            0.,
            0.,
            -0.000787968093807,
            -0.000307591508224,
            -0.000218348633296,
            0.00047738416141,
            0.000120353137658,
            0.000323676257958,
            0.000143850511,
        ];

        #[allow(clippy::excessive_precision, clippy::unreadable_literal)]
        static DIR_COEF_D: &[Float] = &[
            0.,
            0.,
            0.,
            1.60125528528e-05,
            2.9747624976e-06,
            2.31910931569e-05,
            -1.1901034125e-05,
            6.77743734332e-06,
            -5.99548024824e-06,
            -4.79372835035e-06,
        ];
        let mut cache = [0.; DIRECTIVITY_CACHE_SIZE];
        for (idx, dir) in cache.iter_mut().enumerate() {
            let theta_deg = idx as Float * 90. / DIRECTIVITY_CACHE_SIZE as Float;
            let i = (theta_deg / 10.0).ceil() as usize;
            if i == 0 {
                *dir = 1.;
            } else {
                let a = DIR_COEF_A[i];
                let b = DIR_COEF_B[i];
                let c = DIR_COEF_C[i];
                let d = DIR_COEF_D[i];
                let x = theta_deg - (i as Float - 1.0) * 10.0;
                let res = a + b * x + c * x * x + d * x * x * x;
                *dir = res;
            }
        }
        cache
    };
}

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

    pub fn directivity() -> Vec<Float> {
        DIRECTIVITY.to_vec()
    }
}

impl WaveSource for T4010A1 {
    #[inline(always)]
    fn propagate(&self, x: Vector3) -> Complex {
        let diff = crate::fmath::sub(x, self.pos);
        let dist = diff.norm();
        let theta = crate::fmath::acos(self.dir.dot(&diff) / dist);
        let d = {
            let i = (theta * DIRECTIVITY_SLACE).floor() as usize;
            DIRECTIVITY[i]
        };
        let r = self.amp * d * crate::fmath::exp(-dist * self.atten_coef) / dist;
        let phi = self.phase + self.wavenumber * dist;
        Complex::new(r * phi.cos(), r * phi.sin())
    }

    fn set_sound_speed(&mut self, c: Float) {
        self.wavenumber = 2.0 * PI / calc_wavelength(self.frequency, c);
    }

    impl_getset!((get = wavenumber, field = wavenumber), Float);
    impl_getset!((get = position, set = set_position, field = pos), Vector3);
    impl_getset!((get = phase, set = set_phase, field = phase), Float);
    impl_getset!((get = amp, set = set_amp, field = amp), Float);
}

impl std::default::Default for T4010A1 {
    fn default() -> Self {
        Self::new(Vector3::zero(), Vector3::z(), 1., 0., 40e3)
    }
}
