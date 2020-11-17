/*
 * File: builder.rs
 * Project: scatter
 * Created Date: 17/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::scatter_area::ScatterArea;
use crate::{field_type::*, observe_area::builder::*};

use std::{marker::PhantomData, mem::transmute};

/// Builder for ScatterArea
pub struct ScatterAreaBuilder<F> {
    field_type: PhantomData<F>,
}

impl<F> ScatterAreaBuilder<F> {
    pub fn new() -> Self {
        Self {
            field_type: PhantomData,
        }
    }
}

impl ScatterAreaBuilder<Empty> {
    pub fn pressure(self) -> ScatterAreaBuilder<PressureField> {
        unsafe { transmute(self) }
    }

    pub fn power(self) -> ScatterAreaBuilder<PowerField> {
        unsafe { transmute(self) }
    }

    pub fn complex_pressure(self) -> ScatterAreaBuilder<ComplexPressureField> {
        unsafe { transmute(self) }
    }
}

impl<F: FieldType> ScatterAreaBuilder<F> {
    pub fn generate(self) -> ScatterArea<F> {
        ScatterArea::new()
    }
}
