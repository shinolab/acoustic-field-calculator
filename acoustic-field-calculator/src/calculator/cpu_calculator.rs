/*
 * File: calculator.rs
 * Project: calculator
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/06/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::FieldCalculator;
use crate::{
    core::wave_sources::WaveSource, field_buffer::FieldBuffer, observe_area::ObserveArea,
    system::PropagationMedium,
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
}

impl<S, M, A, O, F> FieldCalculator<S, M, A, O, F> for CpuCalculator
where
    S: WaveSource,
    M: PropagationMedium<S>,
    A: ObserveArea,
    O: Send + Sized + Default + Clone,
    F: FieldBuffer<O>,
{
    fn calculate(&self, medium: &M, observe_area: &A, buffer: &mut F) {
        let obs_points = observe_area.points();
        let results = buffer.buffer_mut();
        #[cfg(feature = "parallel")]
        {
            use rayon::{iter::IntoParallelRefIterator, prelude::*};
            obs_points
                .par_iter()
                .map(|&observe_point| {
                    let cp = medium.propagate_all(observe_point);
                    F::calc_from_complex_pressure(cp)
                })
                .collect_into_vec(results);
        }
        #[cfg(not(feature = "parallel"))]
        {
            results.resize(obs_points.len(), Default::default());
            for (result, &observe_point) in results.iter_mut().zip(obs_points) {
                let cp = medium.propagate_all(observe_point);
                *result = F::calc_from_complex_pressure(cp);
            }
        }
    }
}
