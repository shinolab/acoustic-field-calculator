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
    core::container::WaveSourceContainer, field::FieldCalculable, observe_area::ObserveArea,
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
    pub fn calculate<'a, S: WaveSource, A: ObserveArea, T, F: FieldCalculable<T>>(
        &self,
        container: &mut WaveSourceContainer<S>,
        observe_area: &A,
        field: &'a mut F,
    ) {
        field.calculate_field(container, observe_area.observe_points())
    }
}

macro_rules! calc_from_complex_pressure {
    ($wave_sources: ident, $buffer: ident, $val: ident, $f: expr, $results: expr) => {
        #[cfg(feature = "parallel")]
        {
            use rayon::{iter::IntoParallelRefIterator, prelude::*};
            $buffer
                .observe_points()
                .par_iter()
                .map(|&observe_point| {
                    let $val: Complex = $wave_sources
                        .iter()
                        .map(|source| source.propagate(observe_point))
                        .sum();
                    $f
                })
                .collect_into_vec($results);
        }
        #[cfg(not(feature = "parallel"))]
        {
            $results.resize($buffer.observe_points().len(), Default::default());
            for (result, &observe_point) in $results.iter_mut().zip($buffer.observe_points()) {
                let $val: Complex = $wave_sources
                    .iter()
                    .map(|source| source.propagate(observe_point))
                    .sum();
                *result = $f;
            }
        }
    };
}
