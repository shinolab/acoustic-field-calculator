/*
 * File: calculator_acc.rs
 * Project: calculator
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::system_acc::AccPropagationMedium;
use crate::{core::Vector3, field_buffer::FieldBuffer, observe_area::ObserveArea};

/// Accurate Calculator
pub struct AccurateCalculator {}

impl Default for AccurateCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl AccurateCalculator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate<
        M: AccPropagationMedium,
        A: ObserveArea,
        O: Send + Sized + Default + Clone,
        F: FieldBuffer<O>,
    >(
        &self,
        medium: &M,
        observe_area: &A,
        buffer: &mut F,
    ) {
        let obs_points = observe_area.points();
        let results = buffer.buffer_mut();
        self.calculate_at_points::<_, _, F>(medium, obs_points, results);
    }

    pub fn calculate_at_points<
        M: AccPropagationMedium,
        O: Send + Sized + Default + Clone,
        F: FieldBuffer<O>,
    >(
        &self,
        medium: &M,
        obs_points: &[Vector3],
        results: &mut Vec<O>,
    ) {
        #[cfg(feature = "parallel")]
        {
            use rayon::{iter::IntoParallelRefIterator, prelude::*};
            obs_points
                .par_iter()
                .map(|&observe_point| {
                    let cp = medium.propagate(observe_point);
                    F::calc_from_complex_pressure(cp)
                })
                .collect_into_vec(results);
        }
        #[cfg(not(feature = "parallel"))]
        {
            results.resize(obs_points.len(), Default::default());
            for (result, &observe_point) in results.iter_mut().zip(obs_points) {
                let cp = medium.propagate(observe_point);
                *result = F::calc_from_complex_pressure(cp);
            }
        }
    }
}
