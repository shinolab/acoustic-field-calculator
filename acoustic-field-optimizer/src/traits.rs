/*
 * File: traits.rs
 * Project: src
 * Created Date: 21/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

pub use crate::{UniformSystem, WaveSource};

pub trait Optimizer {
    fn optimize<S: WaveSource>(&self, system: &mut UniformSystem<S>);
}
