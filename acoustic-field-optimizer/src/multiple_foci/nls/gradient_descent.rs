/*
 * File: gradient_descent.rs
 * Project: multiple_foci
 * Created Date: 02/10/2020
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

const EPS: Float = 1e-6;
const K_MAX: usize = 10_000;

/// Gradient descent
pub struct GradientDescent {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
}

impl GradientDescent {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>) -> Self {
        Self { foci, amps }
    }
}

impl Optimizer for GradientDescent {
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
        for _ in 0..K_MAX {
            let T = make_T(&x, n, m);
            let Jtf = calc_Jtf(&BhB, &T);
            if Jtf.max() <= EPS {
                break;
            }
            x = &x - &(0.1 * Jtf);
        }

        for (wave_source, &xe) in system.wave_sources_mut().iter_mut().zip(x.iter().take(n)) {
            wave_source.set_phase(xe);
        }
    }
}
