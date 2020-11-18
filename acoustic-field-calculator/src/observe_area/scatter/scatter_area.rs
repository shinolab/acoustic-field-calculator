/*
 * File: scatter_area.rs
 * Project: scatter
 * Created Date: 17/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{core::Vector3, field_type::*, observe_area::traits::*};

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
    fn points_and_results_mut(&mut self) -> (&Vec<Vector3>, &mut Vec<F::Output>) {
        (&self.observe_points, &mut self.results)
    }

    fn results(&self) -> &[F::Output] {
        &self.results
    }

    fn points(&self) -> &[Vector3] {
        &self.observe_points
    }

    fn results_mut(&mut self) -> &mut Vec<F::Output> {
        &mut self.results
    }
}
