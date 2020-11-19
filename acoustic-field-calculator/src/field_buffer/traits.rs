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
    core::{Complex, Float},
    na::ComplexField,
};

pub trait FieldBuffer<T> {
    fn buffer(&self) -> &[T];
    fn buffer_mut(&mut self) -> &mut Vec<T>;
    fn calc_from_complex_pressure(cp: Complex) -> T;
}

pub trait ScalarFieldBuffer<T>: FieldBuffer<T> {
    fn max_result(&self) -> T;
}

impl<T> ScalarFieldBuffer<Float> for T
where
    T: FieldBuffer<Float>,
{
    fn max_result(&self) -> Float {
        self.buffer().iter().fold(Float::NAN, |m, v| v.max(m))
    }
}

impl<T> ScalarFieldBuffer<Complex> for T
where
    T: FieldBuffer<Complex>,
{
    fn max_result(&self) -> Complex {
        self.buffer()
            .iter()
            .fold(Complex::new(Float::NAN, Float::NAN), |m, &v| -> Complex {
                if v.abs() < m.abs() {
                    m
                } else {
                    v
                }
            })
    }
}
