#[macro_use]
extern crate itertools;
extern crate nalgebra as na;
#[macro_use]
extern crate static_assertions;

assert_cfg!(
    not(all(feature = "gpu", feature = "double")),
    "GPU backend cannot be used with double precision float."
);

#[macro_use]
pub mod calculator;
#[macro_use]
mod core;

#[cfg(feature = "accurate")]
#[macro_use]
// accurate mode
pub mod accurate;
#[cfg(feature = "gpu")]
#[macro_use]
// gpu modules
pub mod gpu;

pub mod field_type;
pub mod fmath;
pub mod observe_area;
pub mod prelude;
pub mod system;

pub use crate::core::wave_sources;
pub use crate::core::*;
