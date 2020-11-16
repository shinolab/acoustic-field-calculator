/*
 * File: gs.rs
 * Project: multiple_foci
 * Created Date: 02/10/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/11/2020
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

pub struct GS {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    sound_speed: Float,
}

/// Reference
/// * Asier Marzo and Bruce W Drinkwater. Holographic acoustic tweezers.Proceedings of theNational Academy of Sciences, 116(1):84–89, 2019.
impl GS {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>, sound_speed: Float) -> Self {
        Self {
            foci,
            amps,
            sound_speed,
        }
    }
}

impl Optimizer for GS {
    #[allow(non_snake_case)]
    #[allow(clippy::many_single_char_names)]
    fn optimize<S: WaveSource>(&self, wave_sources: &mut [S]) {
        for source in wave_sources.iter_mut() {
            source.set_phase(0.);
            source.set_sound_speed(self.sound_speed);
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

        let Gh = G.map(|a| a.conj()).transpose();

        let p0 = VectorXcf::from_iterator(m, amps.iter().map(|&a| Complex::new(a, 0.0)));
        let q0 =
            VectorXcf::from_iterator(n, wave_sources.iter().map(|s| Complex::new(s.amp(), 0.)));
        let mut q = q0.clone();

        for _ in 0..REPEAT {
            let gamma = &G * q;
            let p = VectorXcf::from_iterator(
                m,
                gamma.iter().zip(p0.iter()).map(|(g, &p)| g / g.abs() * p),
            );

            let xi = &Gh * p;
            q = VectorXcf::from_iterator(
                n,
                xi.iter().zip(q0.iter()).map(|(x, &q)| x / x.abs() * q),
            );
        }

        for (wave_source, qe) in wave_sources.iter_mut().zip(q.iter()) {
            let amp = qe.abs();
            let phase = qe.argument();
            wave_source.set_amp(amp);
            wave_source.set_phase(phase);
        }
    }
}
