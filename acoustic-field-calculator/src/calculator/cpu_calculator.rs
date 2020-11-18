/*
 * File: calculator.rs
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

use crate::{field_type::FieldType, observe_area::ObserveArea, system::PropagationMedium};

/// Normal Calculator
pub struct CpuCalculator {}

impl Default for CpuCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuCalculator {
    pub fn new() -> Self {
        Self {}
    }

    /// Calculate field at observe area
    ///
    /// # Arguments
    ///
    /// * `observe_area` - Observed area to calculate
    /// * `field` - Field to calculate
    ///
    pub fn calculate<
        'a,
        M: PropagationMedium,
        O: Send,
        F: FieldType<Output = O>,
        A: ObserveArea<F>,
    >(
        &self,
        medium: &'a M,
        observe_area: &'a mut A,
    ) {
        let (obs_points, results) = observe_area.points_and_results_buf();
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
                result = F::calc_from_complex_pressure(cp);
            }
        }
    }
}

macro_rules! propagate {
    ($obs_p: ident, $source: ident, $st: ty) => {{
        let diff = crate::fmath::sub($obs_p, $source.position());
        let dist = diff.norm();
        let theta = crate::fmath::acos($source.direction().dot(&diff) / dist);
        let d = <$st>::directivity(theta);
        let r = $source.amp() * d / dist;
        let phi = $source.phase() + $source.wavenumber() * dist;
        Complex::new(r * phi.cos(), r * phi.sin())
    }};
}

macro_rules! calc_from_complex_pressure {
    ($wave_sources: ident, $st: ty, $obs: ident, $val: ident, $f: expr) => {
        #[cfg(feature = "parallel")]
        {
            use rayon::{iter::IntoParallelRefIterator, prelude::*};
            let (obs_points, results) = $obs.points_and_results_buf();
            obs_points
                .par_iter()
                .map(|&observe_point| {
                    let $val: Complex = $wave_sources
                        .iter()
                        .map(|source| propagate!(observe_point, source, $st))
                        .sum();
                    $f
                })
                .collect_into_vec(results);
        }
        #[cfg(not(feature = "parallel"))]
        {
            let (obs_points, results) = $obs.points_and_results_buf();
            results.resize(obs_points.len(), Default::default());
            for (result, &observe_point) in results.iter_mut().zip(obs_points) {
                let $val: Complex = $wave_sources
                    .iter()
                    .map(|source| propagate!(observe_point, source, $st))
                    .sum();
                *result = $f;
            }
        }
    };
}
