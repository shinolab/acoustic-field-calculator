/*
 * File: gradient_descent.rs
 * Project: multiple_foci
 * Created Date: 02/10/2020
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
type VectorXf = Matrix<Float, Dynamic, U1, VecStorage<Float, Dynamic, U1>>;

const EPS: Float = 1e-6;
const K_MAX: usize = 10_000;

pub struct GradientDescent {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
}

impl GradientDescent {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>) -> Self {
        Self { foci, amps }
    }
}

fn append_matrix_col(to: MatrixXcf, src: &MatrixXcf) -> MatrixXcf {
    assert_eq!(to.nrows(), src.nrows());

    let new_rows = to.nrows();
    let to_cols = to.ncols();
    let new_cols = to.ncols() + src.ncols();

    let mut new_mat = to.resize(new_rows, new_cols, Default::default());
    new_mat
        .slice_mut((0, to_cols), (src.nrows(), src.ncols()))
        .copy_from(src);

    new_mat
}

impl GradientDescent {
    #[allow(non_snake_case)]
    fn make_BhB<S: WaveSource>(
        amps: &[Float],
        foci: &[Vector3],
        wave_sources: &mut [S],
        n: usize,
        m: usize,
    ) -> MatrixXcf {
        let P = MatrixXcf::from_diagonal(&VectorXcf::from_iterator(
            m,
            amps.iter().map(|a| Complex::new(-a, 0.)),
        ));
        let G = MatrixXcf::from_iterator(
            m,
            n,
            wave_sources
                .iter()
                .map(|source| {
                    foci.iter()
                        .map(|&fp| source.propagate(fp))
                        .collect::<Vec<_>>()
                })
                .flatten(),
        );
        let B = append_matrix_col(G, &P);
        B.adjoint() * B
    }

    #[allow(non_snake_case)]
    fn make_T(x: &VectorXf, n: usize, m: usize) -> VectorXcf {
        VectorXcf::from_iterator(n + m, x.iter().map(|x| Complex::new(0., -x).exp()))
    }

    #[allow(non_snake_case)]
    fn calc_Jtf(BhB: &MatrixXcf, T: &VectorXcf) -> VectorXf {
        let TTh = T * T.adjoint();
        let BhB_TTh = BhB.component_mul(&TTh);
        let Jtf = BhB_TTh.map(|c| c.im).column_sum();
        Jtf
    }
}

impl Optimizer for GradientDescent {
    #[allow(non_snake_case, clippy::many_single_char_names)]
    fn optimize<S: WaveSource>(&self, wave_sources: &mut [S]) {
        for source in wave_sources.iter_mut() {
            source.set_phase(0.);
        }

        let num_trans = wave_sources.len();
        let foci = &self.foci;
        let amps = &self.amps;

        let m = foci.len();
        let n = num_trans;

        let n_param = n + m;

        let x0 = VectorXf::zeros(n_param);

        let BhB = Self::make_BhB(amps, foci, wave_sources, n, m);

        let mut x = x0;

        for _ in 0..K_MAX {
            let T = Self::make_T(&x, n, m);
            let Jtf = Self::calc_Jtf(&BhB, &T);
            if Jtf.max() <= EPS {
                break;
            }
            x = &x - &(0.1 * Jtf);
        }

        for (wave_source, &xe) in wave_sources.iter_mut().zip(x.iter().take(n)) {
            wave_source.set_phase(xe);
        }
    }
}
