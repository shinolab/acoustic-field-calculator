/*
 * File: long.rs
 * Project: multiple_foci
 * Created Date: 22/09/2020
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

        let denomi = G.column_sum();
        let X = G
            .map_with_location(|i, _, a| Complex::new(amps[i], 0.0) * a.conj() / denomi[i])
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
                        .zip(amps.iter())
                        .map(|(a, &amp)| a.abs() * amp)
                        .sum()
                })
                .map(|s: Float| Complex::new((s / m as Float).sqrt(), 0.0)),
        ));

        let g = append_matrix_row(G, &sigma);
        let f = VectorXcf::from_iterator(
            m + n,
            amps.iter()
                .zip(e_arg.iter())
                .map(|(amp, &e)| amp * (Complex::new(0., e)).exp())
                .chain((0..n).map(|_| Complex::new(0., 0.))),
        );

        let gt = g.adjoint();
        let gtg = &gt * g;
        let gtf = gt * f;
        let q = gtg.qr().solve(&gtf).unwrap();

        let max_coeff = q.camax();
        for (wave_source, qe) in wave_sources.iter_mut().zip(q.iter()) {
            let amp = wave_source.amp() * qe.abs() / max_coeff;
            let phase = qe.argument();
            wave_source.set_amp(amp);
            wave_source.set_phase(phase);
        }
    }
}
