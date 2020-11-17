/*
 * File: traits.rs
 * Project: field_type
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{core::container::WaveSourceContainer, observe_area::ObserveArea, wave_sources::*};

pub trait FieldType {
    type Output;
}

/// Calculate field by normal calculator
pub trait FieldCalculable: FieldType + Sized {
    fn calculate_field<S: WaveSource, A: ObserveArea<Self>>(
        container: &mut WaveSourceContainer<S>,
        observe_area: &mut A,
    );
}
