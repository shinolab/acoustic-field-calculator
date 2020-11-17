/*
 * File: calculator.rs
 * Project: calculator
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{
    core::container::WaveSourceContainer, field_type::FieldCalculable, observe_area::ObserveArea,
    wave_sources::WaveSource,
};

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
    pub fn calculate<'a, S: WaveSource, F: FieldCalculable, A: ObserveArea<F>>(
        &self,
        container: &mut WaveSourceContainer<S>,
        observe_area: &mut A,
    ) {
        F::calculate_field(container, observe_area)
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
