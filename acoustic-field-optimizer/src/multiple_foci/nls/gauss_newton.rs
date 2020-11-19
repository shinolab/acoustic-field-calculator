/*
 * File: gauss_newton.rs
 * Project: nls
 * Created Date: 03/10/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::macros::*;
use crate::*;

const EPS_1: Float = 1e-8;
const EPS_2: Float = 1e-8;
const K_MAX: usize = 200;

/// Gauss-Newton
pub struct GaussNewton {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
}

impl GaussNewton {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>) -> Self {
        Self { foci, amps }
    }
}

impl Optimizer for GaussNewton {
    #[allow(non_snake_case, clippy::many_single_char_names)]
    fn optimize<S: WaveSource>(&self, system: &mut UniformSystem<S>) {
        for source in system.wave_sources_mut() {
            source.set_phase(0.);
        }

        let m = self.foci.len();
        let n = system.wave_sources().len();

        let n_param = n + m;

        let x0 = VectorXf::zeros(n_param);

        let BhB = make_BhB(system, &self.amps, &self.foci, m);

        let mut x = x0;

        let T = make_T(&x, n, m);
        let (mut A, mut g) = calc_JtJ_Jtf(&BhB, &T);

        let mut found = g.max() <= EPS_1;
        for _ in 0..K_MAX {
            if found {
                break;
            }

            // let h_lm = match A.clone().qr().solve(&g) {
            //     Some(v) => -v,
            //     None => {
            //         break;
            //     }
            // };

            let h_lm = match A.clone().pseudo_inverse(1e-3) {
                Ok(Ai) => -(Ai * &g),
                Err(_) => {
                    break;
                }
            };

            if h_lm.norm() <= EPS_2 * (x.norm() + EPS_2) {
                found = true;
            } else {
                x = &x + &h_lm;
                let T = make_T(&x, n, m);
                let (A_new, g_new) = calc_JtJ_Jtf(&BhB, &T);
                A = A_new;
                g = g_new;
                found = g.max() <= EPS_1;
            }
        }

        for (wave_source, &xe) in system.wave_sources_mut().iter_mut().zip(x.iter().take(n)) {
            wave_source.set_phase(xe);
        }
    }
}
