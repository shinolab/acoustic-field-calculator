/*
 * File: traits.rs
 * Project: observe_area
 * Created Date: 20/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::observe_area::grid::*;
use crate::observe_area::scatter::ScatterArea;
use crate::observe_area::ObserveArea;

pub trait SizedArea: ObserveArea {
    fn size(&self) -> (u32, u32, u32);
}

impl<D> SizedArea for GridArea<D> {
    fn size(&self) -> (u32, u32, u32) {
        let b = self.bounds();
        (b[0] as u32, b[1] as u32, b[2] as u32)
    }
}

impl SizedArea for ScatterArea {
    fn size(&self) -> (u32, u32, u32) {
        (self.observe_points().len() as u32, 1, 1)
    }
}
