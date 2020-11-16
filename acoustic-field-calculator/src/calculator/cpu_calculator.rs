/*
 * File: calculator.rs
 * Project: calculator
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/10/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::WaveSourceContainer;
use crate::core::Float;
use crate::field_buffer::FieldBufferCalculable;
use crate::observe_area::ObserveArea;
use crate::wave_sources::WaveSource;

/// Normal Calculator
pub struct CpuCalculator<S: WaveSource> {
    sources: Vec<S>,
    sound_speed: Float,
}

impl<S: WaveSource> CpuCalculator<S> {
    pub(crate) fn new(sound_speed: Float) -> Self {
        Self {
            sources: vec![],
            sound_speed,
        }
    }
}

impl<S: WaveSource> CpuCalculator<S> {
    /// Calculate field at observe area
    ///
    /// # Arguments
    ///
    /// * `observe_area` - Observed area to calculate
    /// * `field` - Field to calculate
    ///
    pub fn calculate<'a, A: ObserveArea, T, F: FieldBufferCalculable<T>>(
        &mut self,
        observe_area: &A,
        field: &'a mut F,
    ) {
        field.calculate_field(&self, observe_area)
    }
}

impl<S: WaveSource> WaveSourceContainer<S> for CpuCalculator<S> {
    fn wave_sources(&self) -> &[S] {
        &self.sources
    }

    fn wave_sources_mut(&mut self) -> &mut Vec<S> {
        &mut self.sources
    }

    fn sound_speed(&self) -> Float {
        self.sound_speed
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
