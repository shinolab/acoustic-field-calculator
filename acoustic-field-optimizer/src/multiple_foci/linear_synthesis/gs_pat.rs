/*
 * File: gs_pat.rs
 * Project: multiple_foci
 * Created Date: 01/10/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/10/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::Float;
use crate::Optimizer;
use crate::WaveSource;
use crate::{Complex, Vector3};

use na::{ComplexField, Dynamic, Matrix, VecStorage, U1};

type MatrixXcf = Matrix<Complex, Dynamic, Dynamic, VecStorage<Complex, Dynamic, Dynamic>>;
type VectorXcf = Matrix<Complex, Dynamic, U1, VecStorage<Complex, Dynamic, U1>>;
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
    fn optimize<S: WaveSource>(&self, wave_sources: &mut [S]) {
        for source in wave_sources.iter_mut() {
            source.set_phase(0.);
        }

        let num_trans = wave_sources.len();
        let foci = &self.foci;
        let amps = &self.amps;

        let m = foci.len();
        let n = num_trans;

        let G = MatrixXcf::from_iterator(
            m,
            n,
            wave_sources
                .iter()
                .map(|source| {
                    self.foci
                        .iter()
                        .map(|&fp| source.propagate(fp))
                        .collect::<Vec<_>>()
                })
                .flatten(),
        );

        let denomi = G.map(|a| a.abs()).row_sum();
        let B = G
            .map_with_location(|_, j, a| a.conj() / (denomi[j] * denomi[j]))
            .transpose();

        let R = &G * &B;

        let p0 = VectorXcf::from_iterator(m, amps.iter().map(|&a| Complex::new(a, 0.0)));
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

        for (wave_source, qe) in wave_sources.iter_mut().zip(q.iter()) {
            let amp = qe.abs();
            let phase = qe.argument();
            wave_source.set_amp(amp);
            wave_source.set_phase(phase);
        }
    }
}
