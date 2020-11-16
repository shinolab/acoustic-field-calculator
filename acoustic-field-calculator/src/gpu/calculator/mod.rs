/*
 * File: mod.rs
 * Project: calculator
 * Created Date: 19/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

mod builder;
mod gpu_calculator;

pub use gpu_calculator::GpuCalculator;
