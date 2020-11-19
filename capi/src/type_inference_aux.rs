/*
 * File: type_inference_aux.rs
 * Project: src
 * Created Date: 21/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum CalculatorType {
    CpuCalculator = 0,
    AccurateCalculator = 1,
    GpuCalculator = 2,
}
impl CalculatorType {
    pub fn from_i32(x: i32) -> Self {
        FromPrimitive::from_i32(x).unwrap()
    }
}

#[derive(FromPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum SourceType {
    SphereWaveSource = 0,
    T4010A1 = 1,
}
impl SourceType {
    pub fn from_i32(x: i32) -> Self {
        FromPrimitive::from_i32(x).unwrap()
    }
}

#[derive(FromPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum ObserveAreaType {
    GridArea1D = 0,
    GridArea2D = 1,
    GridArea3D = 2,
    Scatter = 3,
}
impl ObserveAreaType {
    pub fn from_i32(x: i32) -> Self {
        FromPrimitive::from_i32(x).unwrap()
    }
}

#[derive(FromPrimitive, Copy, Clone, Debug)]
#[repr(i32)]
pub enum FieldTypes {
    PressureField = 0,
    PowerField = 1,
    ComplexPressureField = 2,
}
impl FieldTypes {
    pub fn from_i32(x: i32) -> Self {
        FromPrimitive::from_i32(x).unwrap()
    }
}
