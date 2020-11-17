/*
 * File: scatter_area.rs
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

use crate::{
    core::{Complex, Float, Vector3},
    field_type::*,
    na::ComplexField,
    observe_area::traits::*,
};

/// Scatter observation points
pub struct ScatterArea<F: FieldType> {
    observe_points: Vec<Vector3>,
    results: Vec<F::Output>,
}

impl<F: FieldType> ScatterArea<F> {
    pub(crate) fn new() -> Self {
        Self {
            observe_points: vec![],
            results: vec![],
        }
    }

    /// Add a new observation point
    ///
    /// # Arguments
    ///
    /// * `v` - Observation point
    pub fn add_observe_point(&mut self, v: Vector3) {
        self.observe_points.push(v);
    }
}

impl<F: FieldType> ObserveArea<F> for ScatterArea<F> {
    fn points_and_results_buf(&mut self) -> (&Vec<Vector3>, &mut Vec<F::Output>) {
        (&self.observe_points, &mut self.results)
    }

    fn results(&self) -> &[F::Output] {
        &self.results
    }
}

impl<F: FieldType<Output = Float>> ScalarFieldBuffer<Float> for ScatterArea<F> {
    fn max_result(&self) -> Float {
        self.results().iter().fold(Float::NAN, |m, v| v.max(m))
    }
}

impl<F: FieldType<Output = Complex>> ScalarFieldBuffer<Complex> for ScatterArea<F> {
    fn max_result(&self) -> Complex {
        self.results.iter().fold(
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
