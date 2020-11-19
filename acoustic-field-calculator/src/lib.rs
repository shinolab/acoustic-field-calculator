#[macro_use]
extern crate itertools;
extern crate nalgebra as na;
#[macro_use]
extern crate static_assertions;

assert_cfg!(
    not(all(feature = "gpu", feature = "double")),
    "GPU backend cannot be used with double precision float."
);

/// Calculator
pub mod calculator;
mod core;

#[cfg(feature = "accurate")]
/// accurate mode
pub mod accurate;
#[cfg(feature = "gpu")]
/// gpu modules
pub mod gpu;

/// Field buffer defines what kind of field to calculate and contains calculation results.
pub mod field_buffer;
/// provide functions of fast math calculation
pub mod fmath;
/// Observe area is a set of observing points
pub mod observe_area;
pub mod prelude;
/// System is an environment where wave sources are located. Currently, this library supports only uniform propagation medium.
pub mod system;

pub use crate::core::wave_sources;
pub use crate::core::*;
