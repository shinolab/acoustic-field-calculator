/*
 * File: ifft.rs
 * Project: multiple_foci
 * Created Date: 02/10/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/02/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::*;

use na::{ComplexField, Dynamic, Matrix, VecStorage};

type MatrixXcf = Matrix<Complex, Dynamic, Dynamic, VecStorage<Complex, Dynamic, Dynamic>>;

use image::GenericImageView;
use rustfft::{FftDirection, FftPlanner};

/// Inverse FFT
pub struct IFFT {
    image_path: String,
    bottom_left: Vector3,
    top_left: Vector3,
    bottom_right: Vector3,
    spacing: Float,
    z: Float,
}

impl IFFT {
    pub fn new(
        path: &str,
        bottom_left: Vector3,
        top_left: Vector3,
        bottom_right: Vector3,
        spacing: Float,
        z: Float,
    ) -> Self {
        Self {
            image_path: path.to_owned(),
            bottom_left,
            top_left,
            bottom_right,
            spacing,
            z,
        }
    }
}

fn fftshift(array: &MatrixXcf, w: usize, h: usize) -> MatrixXcf {
    let mut result = MatrixXcf::zeros(w, h);

    let half_w = (w + 1) >> 1;
    let half_h = (h + 1) >> 1;

    // 2rd to 4th
    result
        .slice_mut((w - half_w, h - half_h), (half_w, half_h))
        .copy_from(&array.slice((0, 0), (half_w, half_h)));

    // 1st to 3rd
    result
        .slice_mut((0, h - half_h), (w - half_w, half_h))
        .copy_from(&array.slice((half_w, 0), (w - half_w, half_h)));

    // 3rd to 1st
    result
        .slice_mut((w - half_w, 0), (half_w, h - half_h))
        .copy_from(&array.slice((0, half_h), (half_w, h - half_h)));

    // 2rd to 4th
    result
        .slice_mut((0, 0), (w - half_w, h - half_h))
        .copy_from(&array.slice((half_w, half_h), (w - half_w, h - half_h)));

    result
}

fn fft2d(array: &mut MatrixXcf, w: usize, h: usize) -> MatrixXcf {
    for i in 0..h {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft(w, FftDirection::Forward);
        fft.process(&mut array.as_mut_slice()[(i * w)..(i * w + w)]);
    }

    let mut result = array.transpose();
    for i in 0..w {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft(h, FftDirection::Forward);
        fft.process(&mut result.as_mut_slice()[(i * h)..(i * h + h)]);
    }

    result.transpose()
}

impl Optimizer for IFFT {
    #[allow(clippy::many_single_char_names)]
    #[allow(non_snake_case)]
    fn optimize<S: WaveSource>(&self, system: &mut UniformSystem<S>) {
        for source in system.wave_sources_mut() {
            source.set_phase(0.);
        }

        let img = image::open(&self.image_path).unwrap();
        let gray = img.grayscale();

        let w = gray.width() as usize;
        let h = gray.height() as usize;

        let max = *gray.to_bytes().iter().max().unwrap() as Float;
        let grat_mat = MatrixXcf::from_iterator(
            w,
            h,
            gray.to_bytes()
                .iter()
                .map(|&v| Complex::new(v as Float / max, 0.)),
        );

        let mut tmp = fftshift(&grat_mat, w, h);
        let tmp = fft2d(&mut tmp, w, h);
        let tmp = fftshift(&tmp, w, h);

        let right = self.bottom_right - self.bottom_left;
        let up = self.top_left - self.bottom_left;
        let center = (right + up) / 2.0;

        let array_w = (right.norm() / self.spacing).ceil() as usize;
        let array_h = (up.norm() / self.spacing).ceil() as usize;

        let right = right.normalize();
        let up = up.normalize();

        let max = tmp.iter().fold(Float::NAN, |m, v| v.abs().max(m));
        let sound_speed = system.sound_speed();
        for source in system.wave_sources_mut() {
            let pos = source.position() - self.bottom_left;

            let x = (right.dot(&pos) / self.spacing).ceil() as isize;
            let y = (up.dot(&pos) / self.spacing).ceil() as isize;
            let x = (w / 2) as isize - (array_w / 2) as isize + x;
            let y = (h / 2) as isize - (array_h / 2) as isize + y;
            if x < 0 || x >= w as isize {
                continue;
            }
            if y < 0 || y >= h as isize {
                continue;
            }
            let x = x as usize;
            let y = y as usize;

            let r = (pos - center + Vector3::new(0., 0., self.z)).norm();

            source.set_amp(source.amp() * tmp[(x, y)].abs() / max);
            source.set_phase(tmp[(x, y)].arg() - 2.0 * PI * source.frequency() / sound_speed * r);
        }
    }
}
