/*
 * File: traits.rs
 * Project: src
 * Created Date: 21/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

pub use acoustic_field_calculator::wave_sources::WaveSource;

pub trait Optimizer {
    fn optimize<S: WaveSource>(&self, wave_sources: &mut [S]);
}
