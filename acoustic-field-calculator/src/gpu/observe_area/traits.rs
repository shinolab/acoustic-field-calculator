/*
 * File: traits.rs
 * Project: observe_area
 * Created Date: 20/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::gpu::field_buffer::GpuFieldType;
use crate::observe_area::{grid::*, scatter::ScatterArea, ObserveArea};

pub trait SizedArea<T, F: GpuFieldType<T>>: ObserveArea<F> {
    fn size(&self) -> (u32, u32, u32);
}

impl<D, T, F: GpuFieldType<T>> SizedArea<T, F> for GridArea<D, F> {
    fn size(&self) -> (u32, u32, u32) {
        let b = self.bounds();
        (b[0] as u32, b[1] as u32, b[2] as u32)
    }
}

impl<T, F: GpuFieldType<T>> SizedArea<T, F> for ScatterArea<F> {
    fn size(&self) -> (u32, u32, u32) {
        (self.points().len() as u32, 1, 1)
    }
}
