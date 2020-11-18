/*
 * File: sound_speed.rs
 * Project: core
 * Created Date: 18/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::core::Float;

pub fn calc_sound_speed(t: Float) -> Float {
    331.3 * (t / 273.15).sqrt() * 1000.0
}
