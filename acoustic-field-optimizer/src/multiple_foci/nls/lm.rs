/*
 * File: lm.rs
 * Project: multiple_foci
 * Created Date: 21/09/2020
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

use na::{Dynamic, Matrix, VecStorage, U1};

type MatrixXcf = Matrix<Complex, Dynamic, Dynamic, VecStorage<Complex, Dynamic, Dynamic>>;
type MatrixXf = Matrix<Float, Dynamic, Dynamic, VecStorage<Float, Dynamic, Dynamic>>;
type VectorXcf = Matrix<Complex, Dynamic, U1, VecStorage<Complex, Dynamic, U1>>;
type VectorXf = Matrix<Float, Dynamic, U1, VecStorage<Float, Dynamic, U1>>;

const EPS_1: Float = 1e-8;
const EPS_2: Float = 1e-8;
const TAU: Float = 1e-3;
const K_MAX: usize = 200;

pub struct LM {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    sound_speed: Float,
}

/// References
/// * K.Levenberg, “A method for the solution of certain non-linear problems in least squares,” Quarterly of applied mathematics, vol.2, no.2, pp.164–168, 1944.
/// * D.W.Marquardt, “An algorithm for least-squares estimation of non-linear parameters,” Journal of the society for Industrial and AppliedMathematics, vol.11, no.2, pp.431–441, 1963.
/// * K.Madsen, H.Nielsen, and O.Tingleff, “Methods for non-linear least squares problems (2nd ed.),” 2004.
impl LM {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>, sound_speed: Float) -> Self {
        Self {
            foci,
            amps,
            sound_speed,
        }
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

impl LM {
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
    fn calc_JtJ_Jtf(BhB: &MatrixXcf, T: &VectorXcf) -> (MatrixXf, VectorXf) {
        let TTh = T * T.adjoint();
        let BhB_TTh = BhB.component_mul(&TTh);
        let JtJ = BhB_TTh.map(|c| c.re);
        let Jtf = BhB_TTh.map(|c| c.im).column_sum();
        (JtJ, Jtf)
    }

    #[allow(non_snake_case)]
    fn calc_Fx(BhB: &MatrixXcf, x: &VectorXf, n: usize, m: usize) -> Float {
        let t = VectorXcf::from_iterator(n + m, x.iter().map(|&x| Complex::new(0., x).exp()));
        (t.adjoint() * BhB * t)[(0, 0)].re
    }
}

impl Optimizer for LM {
    #[allow(non_snake_case, clippy::many_single_char_names)]
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

        let n_param = n + m;

        let x0 = VectorXf::zeros(n_param);

        let I = MatrixXf::identity(n_param, n_param);

        let BhB = Self::make_BhB(amps, foci, wave_sources, n, m);

        let mut x = x0;
        let mut nu = 2.0;

        let T = Self::make_T(&x, n, m);
        let (mut A, mut g) = Self::calc_JtJ_Jtf(&BhB, &T);

        let A_max = A.diagonal().max();
        let mut mu = TAU * A_max;
        let mut found = g.max() <= EPS_1;
        let mut Fx = Self::calc_Fx(&BhB, &x, n, m);
        for _ in 0..K_MAX {
            if found {
                break;
            }

            let h_lm = match (&A + &I.scale(mu)).qr().solve(&g) {
                Some(v) => -v,
                None => {
                    break;
                }
            };
            if h_lm.norm() <= EPS_2 * (x.norm() + EPS_2) {
                found = true;
            } else {
                let x_new = &x + &h_lm;
                let Fx_new = Self::calc_Fx(&BhB, &x_new, n, m);
                let L0_Lhlm = 0.5 * h_lm.dot(&(mu * &h_lm - &g));
                let rho = (Fx - Fx_new) / L0_Lhlm;
                Fx = Fx_new;
                if rho > 0.0 {
                    x = x_new;
                    let T = Self::make_T(&x, n, m);
                    let (A_new, g_new) = Self::calc_JtJ_Jtf(&BhB, &T);
                    A = A_new;
                    g = g_new;
                    found = g.max() <= EPS_1;
                    mu *= (1.0 as Float / 3.).max(1. - (2. * rho - 1.).powf(3.));
                    nu = 2.0;
                } else {
                    mu *= nu;
                    nu *= 2.0;
                }
            }
        }

        for (wave_source, &xe) in wave_sources.iter_mut().zip(x.iter().take(n)) {
            wave_source.set_phase(xe);
        }
    }
}
