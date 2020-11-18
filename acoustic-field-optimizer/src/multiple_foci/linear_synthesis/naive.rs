/*
 * File: naive.rs
 * Project: linear_synthesis
 * Created Date: 03/10/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::multiple_foci::macros::*;
use crate::*;

pub struct Naive {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
}

impl Naive {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>) -> Self {
        Self { foci, amps }
    }
}

impl Optimizer for Naive {
    #[allow(non_snake_case)]
    #[allow(clippy::many_single_char_names)]
    fn optimize<S: WaveSource>(&self, system: &mut UniformSystem<S>) {
        for source in system.wave_sources_mut() {
            source.set_phase(0.);
        }

        let m = self.foci.len();

        let G = generate_propagation_matrix(system, &self.foci);

        let Gh = G.map(|a| a.conj()).transpose();
        let p = VectorXcf::from_iterator(m, self.amps.iter().map(|&a| Complex::new(a, 0.0)));
        let q = Gh * p;

        for (wave_source, qe) in system.wave_sources_mut().iter_mut().zip(q.iter()) {
            let amp = qe.abs();
            let phase = qe.argument();
            wave_source.set_amp(amp);
            wave_source.set_phase(phase);
        }
    }
}
