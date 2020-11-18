/*
 * File: traits.rs
 * Project: buffer
 * Created Date: 08/05/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::field_type::FieldType;
use crate::na::ComplexField;
use crate::{
    core::{Complex, Float, Vector3},
    field_type::*,
};

pub trait ObserveArea<F: FieldType> {
    /// Returns all observation points
    fn points_and_results_buf(&mut self) -> (&Vec<Vector3>, &mut Vec<F::Output>);
    fn results(&self) -> &[F::Output];
}

pub trait ScalarFieldBuffer<F: FieldType> {
    fn max_result(&self) -> F::Output;
}

impl<T> ScalarFieldBuffer<PressureField> for T
where
    T: ObserveArea<PressureField>,
{
    fn max_result(&self) -> Float {
        self.results().iter().fold(Float::NAN, |m, v| v.max(m))
    }
}

impl<T> ScalarFieldBuffer<PowerField> for T
where
    T: ObserveArea<PowerField>,
{
    fn max_result(&self) -> Float {
        self.results().iter().fold(Float::NAN, |m, v| v.max(m))
    }
}

impl<T> ScalarFieldBuffer<ComplexPressureField> for T
where
    T: ObserveArea<ComplexPressureField>,
{
    fn max_result(&self) -> Complex {
        self.results().iter().fold(
            Complex::new(Float::NAN, Float::NAN),
            |m, &v| -> na::Complex<f32> {
                if v.abs() < m.abs() {
                    m
                } else {
                    v
                }
            },
        )
    }
}
