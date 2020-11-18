/*
 * File: mod.rs
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

pub mod attenuation;
pub mod sound_speed;
mod utils;
#[macro_use]
pub mod wave_sources;

#[cfg(not(feature = "double"))]
pub type Float = f32;
#[cfg(not(feature = "double"))]
pub const PI: Float = std::f32::consts::PI;

#[cfg(feature = "double")]
pub type Float = f64;
#[cfg(feature = "double")]
pub const PI: Float = std::f64::consts::PI;

/// Three-dimensional vector
pub type Vector3 = na::Vector3<Float>;
/// Complex number
pub type Complex = na::Complex<Float>;
