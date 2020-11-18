/*
 * File: traits.rs
 * Project: field_type
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{core::wave_sources::WaveSource, field_type::FieldType, system::WaveSourceContainer};
use crate::{gpu::gpu_prelude::*, gpu::*};

/// Calculate field by gpu calculator
pub trait GpuFieldType<T>: FieldType<Output = T> {
    fn calculate_field<
        S: WaveSource,
        M: GpuPropagationMedium<S> + WaveSourceContainer<S>,
        F: GpuFieldType<T>,
        A: SizedArea<T, F>,
    >(
        medium: &M,
        observe_area: &mut A,
        device: GpuDevice,
        queue: GpuQueue,
    );
}
