/*
 * File: traits.rs
 * Project: field_type
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{core::container::WaveSourceContainer, observe_area::*, wave_sources::*};

/// Calculate field by accurate calculator
pub trait AccurateFieldBuffer<D> {
    fn calculate_field<S: WaveSource, F: ObserveArea>(
        &mut self,
        container: &mut WaveSourceContainer<S>,
        field_buffer: &F,
    );
}
