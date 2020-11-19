/*
 * File: foci.rs
 * Project: examples
 * Created Date: 21/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
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
const TRANS_SIZE: Float = 10.18; // mm
const FREQUENCY: Float = 40e3; // Hz
const TEMPERATURE: Float = 300.0; // K

macro_rules! write_image_xy {
    ($filename: tt, $area: ident, $field: ident) => {{
        use image::png::PngEncoder;
        use image::ColorType;
        use scarlet::colormap::ListedColorMap;
        use std::fs::File;

        let bounds = $area.bounds();
        let bb = (bounds.x(), bounds.y());

        let colormap = ListedColorMap::magma();

        let output = File::create($filename).unwrap();
        let max = $field.max_result() as Float;
        let pixels: Vec<_> = $field
            .buffer()
            .chunks_exact(bb.0)
            .rev()
            .flatten()
            .map(|&v| {
                colormap.vals[(v as Float / max * (colormap.vals.len() - 1) as Float) as usize]
            })
            .flat_map(|c| c.iter().map(|x| (x * 255.) as u8).collect::<Vec<_>>())
            .collect();

        let encoder = PngEncoder::new(output);
        encoder
            .encode(&pixels, bb.0 as u32, bb.1 as u32, ColorType::Rgb8)
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

    let mut system = UniformSystem::new(TEMPERATURE);
    for y in 0..NUM_TRANS_Y {
        for x in 0..NUM_TRANS_X {
            let pos = Vector3::new(TRANS_SIZE * x as Float, TRANS_SIZE * y as Float, 0.);
            system.add_wave_source(T4010A1::new(pos, Vector3::z(), 1.0, 0.0, FREQUENCY));
        }
    }

    let r = 200.0;
    let area = GridAreaBuilder::new()
        .x_range(array_center[0] - r / 2.0, array_center[0] + r / 2.0)
        .y_range(array_center[1] - r / 2.0, array_center[1] + r / 2.0)
        .z_at(z)
        .resolution(1.)
        .generate();
    let mut field = PressureField::new();

    let calculator = CpuCalculator::new();

    let foci = vec![
        focal_pos + Vector3::new(50., 0., 0.),
        focal_pos - Vector3::new(50., 0., 0.),
    ];
    let amps = vec![1., 1.];

    /////////////////  IFFT  /////////////////////////
    // can only use when 2D phased array and 2D target
    for entry in WalkDir::new(".").max_depth(4) {
        let entry = entry.unwrap();
        let path = entry.path().to_str().unwrap();
        if path.contains("star.bmp") {
            let bottom_left = Vector3::new(0., 0., 0.);
            let top_left = Vector3::new(0., TRANS_SIZE * (NUM_TRANS_Y - 1) as Float, 0.);
            let bottom_right = Vector3::new(TRANS_SIZE * (NUM_TRANS_X - 1) as Float, 0., 0.);

            IFFT::new(path, bottom_left, top_left, bottom_right, TRANS_SIZE, z)
                .optimize(&mut system);
            calculator.calculate(&system, &area, &mut field);
            write_image_xy!("ifft_star.png", area, field);
            break;
        }
    }

    ////////////////////////// Long et al. ///////////////////////////////
    // please specify maximum amplitude before
    for source in system.wave_sources_mut() {
        source.set_amp(1.0);
    }
    Long::new(foci.clone(), amps.clone()).optimize(&mut system);
    calculator.calculate(&system, &area, &mut field);
    write_image_xy!("long.png", area, field);

    /////////////////////////////// HORN ///////////////////////////////
    // please specify maximum amplitude before
    for source in system.wave_sources_mut() {
        source.set_amp(1.0);
    }
    Horn::new(foci.clone(), amps.clone()).optimize(&mut system);
    calculator.calculate(&system, &area, &mut field);
    write_image_xy!("horn.png", area, field);

    /////////////////  Naive linear synthesis  /////////////////////////
    // please specify maximum amplitude before
    for source in system.wave_sources_mut() {
        source.set_amp(1.0);
    }
    Naive::new(foci.clone(), amps.clone()).optimize(&mut system);
    calculator.calculate(&system, &area, &mut field);
    write_image_xy!("naive.png", area, field);

    /////////////////  Gerchberg-Saxton  /////////////////////////
    // please specify maximum amplitude before
    for source in system.wave_sources_mut() {
        source.set_amp(1.0);
    }
    GS::new(foci.clone(), amps.clone()).optimize(&mut system);
    calculator.calculate(&system, &area, &mut field);
    write_image_xy!("gs.png", area, field);

    /////////////////  GS-PAT  /////////////////////////
    // please specify maximum amplitude before
    for source in system.wave_sources_mut() {
        source.set_amp(1.0);
    }
    GSPAT::new(foci.clone(), amps.clone()).optimize(&mut system);
    calculator.calculate(&system, &area, &mut field);
    write_image_xy!("gs-pat.png", area, field);

    /////////////////  Gradient Descent  /////////////////////////
    // Gradient Descent optimizer currently does not support amplitudes
    for source in system.wave_sources_mut() {
        source.set_amp(1.0);
    }
    GradientDescent::new(foci.clone(), amps.clone()).optimize(&mut system);
    calculator.calculate(&system, &area, &mut field);
    write_image_xy!("gradient.png", area, field);

    /////////////////  Gauss Newton  /////////////////////////
    // Gauss Newton optimizer currently does not support amplitudes
    for source in system.wave_sources_mut() {
        source.set_amp(1.0);
    }
    GaussNewton::new(foci.clone(), amps.clone()).optimize(&mut system);
    calculator.calculate(&system, &area, &mut field);
    write_image_xy!("gauss-newton.png", area, field);

    /////////////////  Levenberg-Marquardt  /////////////////////////
    // Levenberg-Marquardt optimizer currently does not support amplitudes
    for source in system.wave_sources_mut() {
        source.set_amp(1.0);
    }
    LM::new(foci.clone(), amps.clone()).optimize(&mut system);
    calculator.calculate(&system, &area, &mut field);
    write_image_xy!("lm.png", area, field);

    /////////////////  Acoustic Power Optimization  /////////////////////////
    // please specify maximum amplitude before
    for source in system.wave_sources_mut() {
        source.set_amp(1.0);
    }
    APO::new(foci, amps, 2.0).optimize(&mut system);
    calculator.calculate(&system, &area, &mut field);
    write_image_xy!("apo.png", area, field);
}
