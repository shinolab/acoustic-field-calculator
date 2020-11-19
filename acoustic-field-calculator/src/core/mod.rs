/*
 * File: mod.rs
 * Project: core
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

/// utilities for atmospheric attenuation
pub mod attenuation;
/// utilities for sound speed for air
pub mod sound_speed;
/// Wave sources
pub mod wave_sources;

#[cfg(feature = "double")]
mod float {
    /// Floating-point number
    pub type Float = f64;
    pub const PI: Float = std::f64::consts::PI;
}

#[cfg(not(feature = "double"))]
mod float {
    /// Floating-point number
    pub type Float = f32;
    pub const PI: Float = std::f32::consts::PI;
}

pub use float::*;

/// Three-dimensional vector
pub type Vector3 = na::Vector3<Float>;
/// Complex number
pub type Complex = na::Complex<Float>;
