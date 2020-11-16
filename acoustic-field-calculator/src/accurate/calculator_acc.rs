/*
 * File: calculator_acc.rs
 * Project: calculator
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::AccurateFieldBuffer;
use crate::{
    core::container::WaveSourceContainer, observe_area::ObserveArea, wave_sources::WaveSource,
};

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

    /// Calculate field at observe area
    ///
    /// # Arguments
    ///
    /// * `observe_area` - Observed area to calculate
    /// * `field` - Field to calculate
    ///
    pub fn calculate<'a, S: WaveSource, A: ObserveArea, D: Send, T: AccurateFieldBuffer<D>>(
        &self,
        container: &mut WaveSourceContainer<S>,
        observe_area: &A,
        field: &'a mut T,
    ) {
        field.calculate_field(container, observe_area);
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
