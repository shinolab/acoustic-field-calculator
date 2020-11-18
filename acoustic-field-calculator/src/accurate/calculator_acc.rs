/*
 * File: calculator_acc.rs
 * Project: calculator
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::system_acc::AccPropagationMedium;
use crate::{field_type::FieldType, observe_area::ObserveArea};

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
        'a,
        M: AccPropagationMedium,
        O: Send + Sized + Default + Clone,
        F: FieldType<Output = O>,
        A: ObserveArea<F>,
    >(
        &self,
        medium: &'a M,
        observe_area: &'a mut A,
    ) {
        let (obs_points, results) = observe_area.points_and_results_mut();
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
