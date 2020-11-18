/*
 * File: acc_system.rs
 * Project: accurate
 * Created Date: 18/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{
    core::{wave_sources::WaveSource, Complex, Vector3},
    system::UniformSystem,
};
use crate::{system::WaveSourceContainer, Float};
use binary_heap_plus::*;
use ordered_float::OrderedFloat;

pub trait AccPropagationMedium: Sync {
    fn propagate(&self, target: Vector3) -> Complex;
}

impl<S: WaveSource> AccPropagationMedium for UniformSystem<S> {
    fn propagate(&self, target: Vector3) -> Complex {
        let mut re_heap = BinaryHeap::new_min();
        let mut im_heap = BinaryHeap::new_min();

        let sources = self.wave_sources();
        let wavenums = self.wavenums();
        let attens = self.attens();

        re_heap.reserve(sources.len());
        im_heap.reserve(sources.len());

        for i in 0..sources.len() {
            let source = &sources[i];
            let diff = crate::fmath::sub(target, source.position());
            let dist = diff.norm();
            let theta = crate::fmath::acos(source.direction().dot(&diff) / dist);
            let d = S::directivity(theta);
            let r = source.amp() * d * (-dist * attens[i]).exp() / dist;
            let phi = source.phase() + wavenums[i] * dist;
            re_heap.push(OrderedFloat(r * phi.cos()));
            im_heap.push(OrderedFloat(r * phi.sin()));
        }

        let re: Float = re_heap.into_iter_sorted().map(|v| v.into_inner()).sum();
        let im: Float = im_heap.into_iter_sorted().map(|v| v.into_inner()).sum();

        Complex::new(re, im)
    }
}
