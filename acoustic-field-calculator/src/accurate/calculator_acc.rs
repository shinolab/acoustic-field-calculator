/*
 * File: calculator_acc.rs
 * Project: calculator
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/10/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::AccurateFieldBuffer;
use crate::calculator::WaveSourceContainer;
use crate::core::Float;
use crate::observe_area::ObserveArea;
use crate::wave_sources::WaveSource;

/// Accurate Calculator
pub struct AccurateCalculator<S: WaveSource> {
    sources: Vec<S>,
    sound_speed: Float,
}

impl<S: WaveSource> AccurateCalculator<S> {
    pub(crate) fn new(sound_speed: Float) -> Self {
        Self {
            sources: vec![],
            sound_speed,
        }
    }
}

impl<S: WaveSource> AccurateCalculator<S> {
    /// Calculate field at observe area
    ///
    /// # Arguments
    ///
    /// * `observe_area` - Observed area to calculate
    /// * `field` - Field to calculate
    ///
    pub fn calculate<'a, A: ObserveArea, D: Send, T: AccurateFieldBuffer<D>>(
        &mut self,
        observe_area: &A,
        field: &'a mut T,
    ) {
        field.calculate_field(&self, observe_area);
    }
}

impl<S: WaveSource> WaveSourceContainer<S> for AccurateCalculator<S> {
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

macro_rules! calc_from_complex_pressure_accurate {
    ($wave_sources: ident, $buffer: ident, $val: ident, $f: expr, $results: expr) => {
        use crate::Float;
        use binary_heap_plus::*;
        use ordered_float::OrderedFloat;
        #[cfg(feature = "parallel")]
        {
            use rayon::{iter::IntoParallelRefIterator, prelude::*};
            $buffer
                .observe_points()
                .par_iter()
                .map(|&observe_point| {
                    let mut re_heap = BinaryHeap::new_min();
                    let mut im_heap = BinaryHeap::new_min();
                    re_heap.reserve($wave_sources.len());
                    im_heap.reserve($wave_sources.len());
                    for source in $wave_sources.iter() {
                        let c = source.propagate(observe_point);
                        re_heap.push(OrderedFloat(c.re));
                        im_heap.push(OrderedFloat(c.im));
                    }

                    let re: Float = re_heap.into_iter_sorted().map(|v| v.into_inner()).sum();
                    let im: Float = im_heap.into_iter_sorted().map(|v| v.into_inner()).sum();

                    let $val = Complex::new(re, im);
                    $f
                })
                .collect_into_vec($results);
        }
        #[cfg(not(feature = "parallel"))]
        {
            $results.resize($buffer.observe_points().len(), Default::default());
            for (result, &observe_point) in $results.iter_mut().zip($buffer.observe_points()) {
                let mut re_heap = BinaryHeap::new_min();
                let mut im_heap = BinaryHeap::new_min();
                re_heap.reserve($wave_sources.len());
                im_heap.reserve($wave_sources.len());
                for source in $wave_sources.iter() {
                    let c = source.propagate(observe_point);
                    re_heap.push(OrderedFloat(c.re));
                    im_heap.push(OrderedFloat(c.im));
                }

                let re: Float = re_heap.into_iter_sorted().map(|v| v.into_inner()).sum();
                let im: Float = im_heap.into_iter_sorted().map(|v| v.into_inner()).sum();

                let $val = Complex::new(re, im);
                *result = $f;
            }
        }
    };
}
