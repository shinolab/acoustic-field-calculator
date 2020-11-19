/*
 * File: macros.rs
 * Project: multiple_foci
 * Created Date: 18/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::*;
use acoustic_field_calculator::fmath;

pub fn propagate<S: WaveSource>(
    source: &S,
    atten: Float,
    wavenum: Float,
    target: Vector3,
) -> Complex {
    let diff = fmath::sub(target, source.position());
    let dist = diff.norm();
    let theta = fmath::acos(source.direction().dot(&diff) / dist);
    let d = S::directivity(theta);
    let r = source.amp() * d * (-dist * atten).exp() / dist;
    let phi = source.phase() + wavenum * dist;
    Complex::from_polar(r, phi)
}

pub fn generate_propagation_matrix<S: WaveSource>(
    system: &UniformSystem<S>,
    foci: &[Vector3],
) -> MatrixXcf {
    let sources = system.wave_sources();
    let wavenums = system.wavenums();
    let attens = system.attens();

    let m = foci.len();
    let n = sources.len();
    MatrixXcf::from_iterator(
        m,
        n,
        (0..n)
            .map(|i| {
                foci.iter()
                    .map(|&fp| propagate(&sources[i], attens[i], wavenums[i], fp))
                    .collect::<Vec<_>>()
            })
            .flatten(),
    )
}

pub fn append_matrix_col(to: MatrixXcf, src: &MatrixXcf) -> MatrixXcf {
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
