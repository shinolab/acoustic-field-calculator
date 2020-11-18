/*
 * File: mod.rs
 * Project: field_buffer
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

pub mod builder;
/// Observation points on the grid
pub mod grid;
/// Scatter observation points
pub mod scatter;
mod traits;

pub use builder::ObserveAreaBuilder;
pub use grid::GridAreaBuilder;
pub use scatter::ScatterAreaBuilder;
pub use traits::*;
