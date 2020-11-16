/*
 * File: horn.rs
 * Project: multiple_foci
 * Created Date: 27/05/2020
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
use rand::{thread_rng, Rng};

type MatrixXcf = Matrix<Complex, Dynamic, Dynamic, VecStorage<Complex, Dynamic, Dynamic>>;
type VectorXcf = Matrix<Complex, Dynamic, U1, VecStorage<Complex, Dynamic, U1>>;

const REPEAT_SDP: usize = 100;
const LAMBDA_SDP: Float = 0.8;
const TIKHONOV_DEFAULT: Float = 1e-5;

pub struct Horn {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    repeat: usize,
    lambda: Float,
    tikhonov_parameter: Float,
    sound_speed: Float,
}

/// Reference
/// * Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE World Haptics Conference (WHC). IEEE, 2015.
impl Horn {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>, sound_speed: Float) -> Self {
        Self {
            foci,
            amps,
            repeat: REPEAT_SDP,
            lambda: LAMBDA_SDP,
            tikhonov_parameter: TIKHONOV_DEFAULT,
            sound_speed,
        }
    }

    pub fn set_repeat(&mut self, repeat: usize) {
        self.repeat = repeat;
    }

    pub fn set_lambda(&mut self, lambda: Float) {
        self.lambda = lambda;
    }

    pub fn set_tikhonov_param(&mut self, a: Float) {
        self.tikhonov_parameter = a;
    }
}

fn pseudo_inverse_with_reg(m: &MatrixXcf, alpha: Float) -> MatrixXcf {
    let svd = m.clone().svd(true, true);
    let s_inv = MatrixXcf::from_diagonal(
        &svd.singular_values
            .map(|s| Complex::new(s / (s * s + alpha * alpha), 0.)),
    );
    match (&svd.v_t, &svd.u) {
        (Some(v_t), Some(u)) => v_t.adjoint() * s_inv * u.adjoint(),
        _ => unreachable!(),
    }
}

impl Optimizer for Horn {
    #[allow(clippy::many_single_char_names)]
    #[allow(non_snake_case)]
    fn optimize<S: WaveSource>(&self, wave_sources: &mut [S]) {
        for source in wave_sources.iter_mut() {
            source.set_phase(0.);
            source.set_sound_speed(self.sound_speed);
        }

        let m = self.foci.len();
        let n = wave_sources.len();

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
        let P = MatrixXcf::from_diagonal(&VectorXcf::from_iterator(
            m,
            self.amps.iter().map(|&a| Complex::new(a, 0.)),
        ));

        let G_pinv = pseudo_inverse_with_reg(&G, self.tikhonov_parameter);
        let MM = &P * (MatrixXcf::identity(m, m) - G * &G_pinv) * &P;
        let mut X = MatrixXcf::identity(m, m);

        let mut rng = thread_rng();
        let lambda = self.lambda;
        for _ in 0..(m * self.repeat) {
            let ii = (m as f32 * rng.gen_range(0., 1.)) as usize;
            let Xc = X.clone().remove_row(ii).remove_column(ii);
            let MMc = MM.column(ii).remove_row(ii);
            let Xb = Xc * &MMc;
            let gamma = (Xb.adjoint() * MMc)[(0, 0)];
            if gamma.re > 0. {
                let Xb = Xb.scale(-(lambda / gamma.re).sqrt());
                X.slice_mut((ii, 0), (1, ii))
                    .copy_from(&Xb.slice((0, 0), (ii, 1)).adjoint());
                X.slice_mut((ii, ii + 1), (1, m - ii - 1))
                    .copy_from(&Xb.slice((ii, 0), (m - 1 - ii, 1)).adjoint());
                X.slice_mut((0, ii), (ii, 1))
                    .copy_from(&Xb.slice((0, 0), (ii, 1)));
                X.slice_mut((ii + 1, ii), (m - ii - 1, 1))
                    .copy_from(&Xb.slice((ii, 0), (m - 1 - ii, 1)));
            } else {
                let z1 = VectorXcf::zeros(ii);
                let z2 = VectorXcf::zeros(m - ii - 1);
                X.slice_mut((ii, 0), (1, ii)).copy_from(&z1.adjoint());
                X.slice_mut((ii, ii + 1), (1, m - ii - 1))
                    .copy_from(&z2.adjoint());
                X.slice_mut((0, ii), (ii, 1)).copy_from(&z1);
                X.slice_mut((ii + 1, ii), (m - ii - 1, 1)).copy_from(&z2);
            }
        }

        let eig = na::SymmetricEigen::new(X);
        let u = eig.eigenvectors.column(eig.eigenvalues.imax());
        let q = G_pinv * P * u;
        let max_coeff = q.camax();
        for (wave_source, qe) in wave_sources.iter_mut().zip(q.iter()) {
            let amp = wave_source.amp() * qe.abs() / max_coeff;
            let phase = qe.argument();
            wave_source.set_amp(amp);
            wave_source.set_phase(phase);
        }
    }
}
