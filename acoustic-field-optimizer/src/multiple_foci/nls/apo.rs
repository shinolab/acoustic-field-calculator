/*
 * File: apo.rs
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

use crate::multiple_foci::macros::*;
use crate::*;

const EPS: Float = 1e-8;
const K_MAX: usize = 200;
const LINE_SEARCH_MAX: usize = 100;

/// Acoustic Power Optimization
pub struct APO {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    lambda: Float,
}

/// References
/// * Keisuke Hasegawa, Hiroyuki Shinoda, and Takaaki Nara. Volumetric acoustic holography andits application to self-positioning by single channel measurement.Journal of Applied Physics,127(24):244904, 2020.7
impl APO {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>, lambda: Float) -> Self {
        Self { foci, amps, lambda }
    }
}

impl APO {
    #[allow(non_snake_case)]
    fn make_Ri(G: &MatrixXcf, i: usize, m: usize) -> MatrixXcf {
        let mut Di = MatrixXcf::zeros(m, m);
        Di[(i, i)] = Complex::new(1., 0.);
        G.adjoint() * Di * G
    }

    #[allow(non_snake_case)]
    fn calc_J(p2: &VectorXf, q: &VectorXcf, Ris: &[MatrixXcf], m: usize, lambda: Float) -> Float {
        (0..m)
            .map(|i| {
                let s = (q.adjoint() * &Ris[i] * q)[0] - p2[i];
                s.norm_squared()
            })
            .sum::<Float>()
            + q.dot(&q).abs() * lambda
    }

    #[allow(non_snake_case)]
    fn calc_nabla_J(
        p2: &VectorXf,
        q: &VectorXcf,
        Ris: &[MatrixXcf],
        m: usize,
        lambda: Float,
    ) -> VectorXcf {
        (0..m)
            .map(|i| {
                let s = p2[i] - (q.adjoint() * &Ris[i] * q)[0].abs();
                (&Ris[i] * q).scale(s)
            })
            .sum::<VectorXcf>()
            + q.scale(lambda)
    }

    // Does not consider Wolfe-Powell condition
    // Only search alpha in [0,1)
    #[allow(non_snake_case)]
    #[allow(clippy::many_single_char_names)]
    fn line_search(
        q: &VectorXcf,
        d: &VectorXcf,
        p2: &VectorXf,
        Ris: &[MatrixXcf],
        m: usize,
        lambda: Float,
    ) -> Float {
        let mut alpha = 0.;
        let mut min = Float::INFINITY;

        for i in 0..LINE_SEARCH_MAX {
            let a = i as Float / LINE_SEARCH_MAX as Float;
            let v = Self::calc_J(p2, &(q + d.scale(a)), Ris, m, lambda);
            if v < min {
                alpha = a;
                min = v;
            }
        }

        alpha
    }
}

impl Optimizer for APO {
    #[allow(non_snake_case, clippy::many_single_char_names)]
    fn optimize<S: WaveSource>(&self, system: &mut UniformSystem<S>) {
        for source in system.wave_sources_mut() {
            source.set_phase(0.);
        }

        let m = self.foci.len();
        let n = system.wave_sources().len();

        let G = generate_propagation_matrix(system, &self.foci);

        let p = VectorXcf::from_iterator(m, self.amps.iter().map(|&a| Complex::new(a, 0.)));
        let p2 = p.map(|v| v.norm_squared());

        let I = MatrixXcf::identity(n, n);
        let q0 = (G.adjoint() * &G + I.scale(self.lambda))
            .qr()
            .solve(&(G.adjoint() * &p))
            .unwrap();
        let Ris: Vec<_> = (0..m).map(|i| Self::make_Ri(&G, i, m)).collect();

        let mut H = I;
        let mut q = q0;

        let mut nabla_J = Self::calc_nabla_J(&p2, &q, &Ris, m, self.lambda);
        for _ in 0..K_MAX {
            let d = -(&H * &nabla_J);

            // let alpha: Float = 0.01;
            let alpha = Self::line_search(&q, &d, &p2, &Ris, m, self.lambda);

            let d = d.scale(alpha);

            if d.norm() < EPS {
                break;
            }

            let q_new = &q + &d;
            let nabla_J_new = Self::calc_nabla_J(&p2, &q_new, &Ris, m, self.lambda);

            let s = &nabla_J_new - nabla_J;
            let y = d;

            H = &H + &y * y.transpose() / y.dot(&s)
                - (&H * &s * s.transpose() * H.transpose()) / ((s.transpose() * &H * s)[0]);

            q = q_new;
            nabla_J = nabla_J_new;
        }

        let max_coeff = q.camax();
        for (wave_source, qe) in system.wave_sources_mut().iter_mut().zip(q.iter()) {
            let amp = wave_source.amp() * qe.abs() / max_coeff;
            let phase = qe.argument();
            wave_source.set_amp(amp);
            wave_source.set_phase(phase);
        }
    }
}
