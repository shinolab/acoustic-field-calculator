/*
 * File: gs_pat.rs
 * Project: multiple_foci
 * Created Date: 01/10/2020
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

const REPEAT: usize = 100;

pub struct GSPAT {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
}

/// Reference
/// * Diego Martinez Plasencia et al. "Gs-pat: high-speed multi-point sound-fields for phased arrays of transducers," ACMTrans-actions on Graphics (TOG), 39(4):138–1, 2020.
///
/// Not yet been implemented with GPU.
impl GSPAT {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>) -> Self {
        Self { foci, amps }
    }
}

impl Optimizer for GSPAT {
    #[allow(non_snake_case)]
    #[allow(clippy::many_single_char_names)]
    fn optimize<S: WaveSource>(&self, system: &mut UniformSystem<S>) {
        for source in system.wave_sources_mut() {
            source.set_phase(0.);
        }

        let m = self.foci.len();

        let G = generate_propagation_matrix(system, &self.foci);

        let denomi = G.map(|a| a.abs()).row_sum();
        let B = G
            .map_with_location(|_, j, a| a.conj() / (denomi[j] * denomi[j]))
            .transpose();

        let R = &G * &B;

        let p0 = VectorXcf::from_iterator(m, self.amps.iter().map(|&a| Complex::new(a, 0.0)));
        let mut p = p0.clone();
        let mut gamma = &R * p;

        for _ in 0..REPEAT {
            p = VectorXcf::from_iterator(
                m,
                gamma.iter().zip(p0.iter()).map(|(g, &p)| g / g.abs() * p),
            );
            gamma = &R * p;
        }
        p = VectorXcf::from_iterator(
            m,
            gamma
                .iter()
                .zip(p0.iter())
                .map(|(g, &p)| g / (g.abs() * g.abs()) * p * p),
        );

        let q = B * p;

        for (wave_source, qe) in system.wave_sources_mut().iter_mut().zip(q.iter()) {
            let amp = qe.abs();
            let phase = qe.argument();
            wave_source.set_amp(amp);
            wave_source.set_phase(phase);
        }
    }
}
