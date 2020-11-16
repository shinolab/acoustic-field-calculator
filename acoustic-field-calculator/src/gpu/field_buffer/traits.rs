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

use crate::gpu::gpu_prelude::*;
use crate::gpu::*;

/// Calculate field by gpu calculator
pub trait GpuFieldBuffer {
    fn calculate_field<S: GpuWaveSource, F: SizedArea>(
        &mut self,
        calculator: &mut GpuCalculator<S>,
        field_buffer: &F,
        device: GpuDevice,
        queue: GpuQueue,
    );
}
