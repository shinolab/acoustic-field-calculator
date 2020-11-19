/*
 * File: traits.rs
 * Project: field_type
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{
    core::wave_sources::WaveSource, field_buffer::FieldBuffer, system::WaveSourceContainer,
};
use crate::{gpu::gpu_prelude::*, gpu::*};

/// Calculate field by gpu calculator
pub trait GpuFieldBuffer<T>: FieldBuffer<T> {
    fn calculate_field<
        S: WaveSource,
        M: GpuPropagationMedium<S> + WaveSourceContainer<S>,
        F: GpuFieldBuffer<T>,
        A: SizedArea,
    >(
        medium: &M,
        observe_area: &A,
        buffer: &mut F,
        device: GpuDevice,
        queue: GpuQueue,
    );
}
