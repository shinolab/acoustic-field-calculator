/*
 * File: lm.rs
 * Project: multiple_foci
 * Created Date: 21/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::macros::*;
use crate::*;

const EPS_1: Float = 1e-8;
const EPS_2: Float = 1e-8;
const TAU: Float = 1e-3;
const K_MAX: usize = 200;

pub struct LM {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
}

/// References
/// * K.Levenberg, “A method for the solution of certain non-linear problems in least squares,” Quarterly of applied mathematics, vol.2, no.2, pp.164–168, 1944.
/// * D.W.Marquardt, “An algorithm for least-squares estimation of non-linear parameters,” Journal of the society for Industrial and AppliedMathematics, vol.11, no.2, pp.431–441, 1963.
/// * K.Madsen, H.Nielsen, and O.Tingleff, “Methods for non-linear least squares problems (2nd ed.),” 2004.
impl LM {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>) -> Self {
        Self { foci, amps }
    }
}

impl Optimizer for LM {
    #[allow(non_snake_case, clippy::many_single_char_names)]
    fn optimize<S: WaveSource>(&self, system: &mut UniformSystem<S>) {
        for source in system.wave_sources_mut() {
            source.set_phase(0.);
        }

        let m = self.foci.len();
        let n = system.wave_sources().len();

        let n_param = n + m;

        let x0 = VectorXf::zeros(n_param);

        let I = MatrixXf::identity(n_param, n_param);

        let BhB = make_BhB(system, &self.amps, &self.foci, m);

        let mut x = x0;
        let mut nu = 2.0;

        let T = make_T(&x, n, m);
        let (mut A, mut g) = calc_JtJ_Jtf(&BhB, &T);

        let A_max = A.diagonal().max();
        let mut mu = TAU * A_max;
        let mut found = g.max() <= EPS_1;
        let mut Fx = calc_Fx(&BhB, &x, n, m);
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
                let Fx_new = calc_Fx(&BhB, &x_new, n, m);
                let L0_Lhlm = 0.5 * h_lm.dot(&(mu * &h_lm - &g));
                let rho = (Fx - Fx_new) / L0_Lhlm;
                Fx = Fx_new;
                if rho > 0.0 {
                    x = x_new;
                    let T = make_T(&x, n, m);
                    let (A_new, g_new) = calc_JtJ_Jtf(&BhB, &T);
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

        for (wave_source, &xe) in system.wave_sources_mut().iter_mut().zip(x.iter().take(n)) {
            wave_source.set_phase(xe);
        }
    }
}
