/*
 * File: foci.rs
 * Project: examples
 * Created Date: 21/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */
use acoustic_field_calculator::prelude::*;
use acoustic_field_optimizer::multiple_foci::*;
use acoustic_field_optimizer::*;

use walkdir::WalkDir;

const NUM_TRANS_X: usize = 18;
const NUM_TRANS_Y: usize = 14;
const TRANS_SIZE: Float = 10.18;
const FREQUENCY: Float = 40e3;
const SOUND_SPEED: Float = 340e3;

macro_rules! write_image {
    ($filename: tt, $field: ident, $bb: ident) => {{
        use image::png::PngEncoder;
        use image::ColorType;
        use scarlet::colormap::ListedColorMap;
        use std::fs::File;

        let colormap = ListedColorMap::magma();

        let output = File::create($filename).unwrap();
        let max = $field.max() as Float;
        let pixels: Vec<_> = $field
            .buffer()
            .chunks_exact($bb.0)
            .rev()
            .flatten()
            .map(|&v| {
                colormap.vals[(v as Float / max * (colormap.vals.len() - 1) as Float) as usize]
            })
            .flat_map(|c| c.iter().map(|x| (x * 255.) as u8).collect::<Vec<_>>())
            .collect();

        let encoder = PngEncoder::new(output);
        encoder
            .encode(&pixels, $bb.0 as u32, $bb.1 as u32, ColorType::Rgb8)
            .unwrap();
    }};
}

fn main() {
    let array_center = Vector3::new(
        TRANS_SIZE * (NUM_TRANS_X - 1) as Float / 2.0,
        TRANS_SIZE * (NUM_TRANS_Y - 1) as Float / 2.0,
        0.,
    );

    let z = 150.0;
    let focal_pos = array_center + z * Vector3::z();

    let mut calculator = CalculatorBuilder::new()
        .set_sound_speed(SOUND_SPEED)
        .generate();

    let amp = 1.0;
    for y in 0..NUM_TRANS_Y {
        for x in 0..NUM_TRANS_X {
            let pos = Vector3::new(TRANS_SIZE * x as Float, TRANS_SIZE * y as Float, 0.);
            calculator.add_wave_source(T4010A1::new(pos, Vector3::z(), amp, 0.0, FREQUENCY));
        }
    }

    let r = 100.0;
    let area = GridAreaBuilder::new()
        .x_range(array_center[0] - r, array_center[0] + r)
        .y_range(array_center[1] - r, array_center[1] + r)
        .z_at(z)
        .resolution(1.)
        .generate();

    let mut field = PressureField::new();
    let foci = vec![
        focal_pos + Vector3::new(50., 0., 0.),
        focal_pos - Vector3::new(50., 0., 0.),
    ];
    let amps = vec![1., 1.];

    /////////////////  IFFT  /////////////////////////
    // can only use when 2d phased array and 2d target
    for entry in WalkDir::new(".").max_depth(4) {
        let entry = entry.unwrap();
        let path = entry.path().to_str().unwrap();
        if path.contains("star.bmp") {
            let bottom_left = Vector3::new(0., 0., 0.);
            let top_left = Vector3::new(0., TRANS_SIZE * (NUM_TRANS_Y - 1) as Float, 0.);
            let bottom_right = Vector3::new(TRANS_SIZE * (NUM_TRANS_X - 1) as Float, 0., 0.);

            IFFT::new(path, bottom_left, top_left, bottom_right, TRANS_SIZE, z)
                .optimize(calculator.wave_sources_mut());
            calculator.calculate(&area, &mut field);
            let bounds = area.bounds();
            let bb = (bounds.x(), bounds.y());
            write_image!("xy_ifft.png", field, bb);
        }
    }

    ////////////////////////// Long et al. ///////////////////////////////
    // please specify maximum amplitude before
    for source in calculator.wave_sources_mut().iter_mut() {
        source.set_amp(1.0);
    }
    Long::new(foci.clone(), amps.clone()).optimize(calculator.wave_sources_mut());
    calculator.calculate(&area, &mut field);
    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy_long.png", field, bb);

    /////////////////////////////// HORN ///////////////////////////////
    // please specify maximum amplitude before
    for source in calculator.wave_sources_mut().iter_mut() {
        source.set_amp(1.0);
    }
    Horn::new(foci.clone(), amps.clone()).optimize(calculator.wave_sources_mut());
    calculator.calculate(&area, &mut field);
    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy_horn.png", field, bb);

    /////////////////  Naive linear synthesis  /////////////////////////
    // please specify maximum amplitude before
    for source in calculator.wave_sources_mut().iter_mut() {
        source.set_amp(1.0);
    }
    Naive::new(foci.clone(), amps.clone()).optimize(calculator.wave_sources_mut());
    calculator.calculate(&area, &mut field);
    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy_naive.png", field, bb);

    /////////////////  Gerchberg-Saxton  /////////////////////////
    // please specify maximum amplitude before
    for source in calculator.wave_sources_mut().iter_mut() {
        source.set_amp(1.0);
    }
    GS::new(foci.clone(), amps.clone()).optimize(calculator.wave_sources_mut());
    calculator.calculate(&area, &mut field);
    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy_gs.png", field, bb);

    /////////////////  GS-PAT  /////////////////////////
    // please specify maximum amplitude before
    for source in calculator.wave_sources_mut().iter_mut() {
        source.set_amp(1.0);
    }
    GSPAT::new(foci.clone(), amps.clone()).optimize(calculator.wave_sources_mut());
    calculator.calculate(&area, &mut field);
    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy_gspat.png", field, bb);

    /////////////////  Gradient Descent  /////////////////////////
    // Gradient Descent optimizer currently does not support amplitudes
    for source in calculator.wave_sources_mut().iter_mut() {
        source.set_amp(1.0);
    }
    GradientDescent::new(foci.clone(), amps.clone()).optimize(calculator.wave_sources_mut());
    calculator.calculate(&area, &mut field);
    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy_gradient.png", field, bb);

    /////////////////  Gauss Newton  /////////////////////////
    // Gauss Newton optimizer currently does not support amplitudes
    for source in calculator.wave_sources_mut().iter_mut() {
        source.set_amp(1.0);
    }
    GaussNewton::new(foci.clone(), amps.clone()).optimize(calculator.wave_sources_mut());
    calculator.calculate(&area, &mut field);
    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy_gauss-newton.png", field, bb);

    /////////////////  Levenberg-Marquardt  /////////////////////////
    // Levenberg-Marquardt optimizer currently does not support amplitudes
    for source in calculator.wave_sources_mut().iter_mut() {
        source.set_amp(1.0);
    }
    LM::new(foci.clone(), amps.clone()).optimize(calculator.wave_sources_mut());
    calculator.calculate(&area, &mut field);
    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy_lm.png", field, bb);

    /////////////////  Acoustic Power Optimization  /////////////////////////
    // please specify maximum amplitude before
    for source in calculator.wave_sources_mut().iter_mut() {
        source.set_amp(1.0);
    }
    APO::new(foci, amps, 2.0).optimize(calculator.wave_sources_mut());
    calculator.calculate(&area, &mut field);
    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy_apo.png", field, bb);
}
