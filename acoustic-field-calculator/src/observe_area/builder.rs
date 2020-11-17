/*
 * File: builder.rs
 * Project: observe_area
 * Created Date: 17/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::grid::GridAreaBuilder;
use super::scatter::ScatterAreaBuilder;
use crate::field_type::*;

use std::marker::PhantomData;
use std::mem::transmute;

pub struct Empty;
pub struct Unused;
pub struct Used;

pub struct Grid;
pub struct Scatter;

/// Builder for GridArea
pub struct ObserveAreaBuilder<A, F> {
    area_type: PhantomData<A>,
    field_type: PhantomData<F>,
}

impl<F> ObserveAreaBuilder<Empty, F> {
    pub fn new() -> Self {
        Self {
            area_type: PhantomData,
            field_type: PhantomData,
        }
    }

    pub fn grid(self) -> GridAreaBuilder<Empty, Empty, Empty, Empty, F> {
        GridAreaBuilder::new()
    }
    pub fn scatter(self) -> ScatterAreaBuilder<F> {
        ScatterAreaBuilder::new()
    }
}

impl<A> ObserveAreaBuilder<A, Empty> {
    pub fn pressure(self) -> ObserveAreaBuilder<A, PressureField> {
        unsafe { transmute(self) }
    }

    pub fn power(self) -> ObserveAreaBuilder<A, PowerField> {
        unsafe { transmute(self) }
    }

    pub fn complex_pressure(self) -> ObserveAreaBuilder<A, ComplexPressureField> {
        unsafe { transmute(self) }
    }
}
