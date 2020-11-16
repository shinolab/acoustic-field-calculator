/*
 * File: dimension.rs
 * Project: grid_area
 * Created Date: 20/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Dimension {
    None,
    One(Axis),
    Two(Axis, Axis),
    Three(Axis, Axis, Axis),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Axis {
    None = 0,
    X = 1,
    Y = 2,
    Z = 3,
}

impl Axis {
    pub fn from_i32(x: i32) -> Self {
        match x {
            1 => Axis::X,
            2 => Axis::Y,
            3 => Axis::Z,
            _ => Axis::None,
        }
    }
}
