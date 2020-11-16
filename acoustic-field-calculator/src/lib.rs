#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate nalgebra as na;
#[macro_use]
extern crate static_assertions;

assert_cfg!(
    not(all(feature = "gpu", feature = "double")),
    "GPU backend cannot be used with double precision float."
);

#[cfg(feature = "accurate")]
#[macro_use]
/// accurate mode
pub mod accurate;
#[macro_use]
pub mod calculator;
#[macro_use]
mod core;
pub mod field;
#[cfg(feature = "gpu")]
#[macro_use]
/// gpu modules
pub mod gpu;

pub mod fmath;
pub mod observe_area;
pub mod prelude;

pub use crate::core::wave_sources;
pub use crate::core::*;
