/*
 * File: traits.rs
 * Project: field_type
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::calculator::*;
use crate::observe_area::*;
use crate::wave_sources::*;

pub trait FieldBuffer<D> {
    fn buffer(&self) -> &[D];
    fn buffer_mut(&mut self) -> &mut Vec<D>;
}

/// Calculate field by normal calculator
pub trait FieldBufferCalculable<D>: FieldBuffer<D> {
    fn calculate_field<S: WaveSource, F: ObserveArea>(
        &mut self,
        calculator: &CpuCalculator<S>,
        field_buffer: &F,
    );
}

pub trait ScalarFieldBuffer<D>: FieldBuffer<D> {
    fn max(&self) -> D;
}
