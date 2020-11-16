/*
 * File: mod.rs
 * Project: fmath
 * Created Date: 22/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

mod exp;
mod simd_vec;
mod trigonometric;

pub use exp::exp;
pub use simd_vec::sub;
pub use trigonometric::*;
