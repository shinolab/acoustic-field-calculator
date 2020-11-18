/*
 * File: long.rs
 * Project: multiple_foci
 * Created Date: 22/09/2020
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

pub struct Long {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
}

/// Reference
/// * Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air using ultrasound." ACM Transactions on Graphics (TOG) 33.6 (2014): 1-10.
impl Long {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>) -> Self {
        Self { foci, amps }
    }
}

fn append_matrix_row(to: MatrixXcf, src: &MatrixXcf) -> MatrixXcf {
    assert_eq!(to.ncols(), src.ncols());

    let new_cols = to.ncols();
    let to_rows = to.nrows();
    let new_rows = to.nrows() + src.nrows();

    let mut new_mat = to.resize(new_rows, new_cols, Default::default());
    new_mat
        .slice_mut((to_rows, 0), (src.nrows(), src.ncols()))
        .copy_from(src);

    new_mat
}

impl Optimizer for Long {
    #[allow(non_snake_case)]
    #[allow(clippy::many_single_char_names)]
    fn optimize<S: WaveSource>(&self, system: &mut UniformSystem<S>) {
        for source in system.wave_sources_mut() {
            source.set_phase(0.);
        }

        let m = self.foci.len();
        let n = system.wave_sources().len();

        let G = generate_propagation_matrix(system, &self.foci);

        let denomi = G.column_sum();
        let X = G
            .map_with_location(|i, _, a| Complex::new(self.amps[i], 0.0) * a.conj() / denomi[i])
            .transpose();

        let R = &G * X;

        let eig = R.symmetric_eigen();
        let e_arg = eig
            .eigenvectors
            .row(eig.eigenvalues.imax())
            .map(|e| e.argument());

        let sigma = MatrixXcf::from_diagonal(&VectorXcf::from_iterator(
            n,
            G.column_iter()
                .map(|col| {
                    col.iter()
                        .zip(self.amps.iter())
                        .map(|(a, &amp)| a.abs() * amp)
                        .sum()
                })
                .map(|s: Float| Complex::new((s / m as Float).sqrt(), 0.0)),
        ));

        let g = append_matrix_row(G, &sigma);
        let f = VectorXcf::from_iterator(
            m + n,
            self.amps
                .iter()
                .zip(e_arg.iter())
                .map(|(amp, &e)| amp * (Complex::new(0., e)).exp())
                .chain((0..n).map(|_| Complex::new(0., 0.))),
        );

        let gt = g.adjoint();
        let gtg = &gt * g;
        let gtf = gt * f;
        let q = gtg.qr().solve(&gtf).unwrap();

        let max_coeff = q.camax();
        for (wave_source, qe) in system.wave_sources_mut().iter_mut().zip(q.iter()) {
            let amp = wave_source.amp() * qe.abs() / max_coeff;
            let phase = qe.argument();
            wave_source.set_amp(amp);
            wave_source.set_phase(phase);
        }
    }
}
