/*
 * File: simple.rs
 * Project: examples
 * Created Date: 27/05/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use acoustic_field_optimizer::*;

const NUM_TRANS_X: usize = 18;
const NUM_TRANS_Y: usize = 14;
const TRANS_SIZE: Float = 10.18; // mm
const FREQUENCY: Float = 40e3; // Hz
const TEMPERATURE: Float = 300.0; // K

macro_rules! write_image {
    ($filename: tt, $field: ident, $bb: ident) => {{
        use image::png::PngEncoder;
        use image::ColorType;
        use scarlet::colormap::ListedColorMap;
        use std::fs::File;

        let colormap = ListedColorMap::magma();

        let output = File::create($filename).unwrap();
        let max = $field.max_result() as Float;
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

    let mut system = UniformSystem::new(TEMPERATURE);
    for y in 0..NUM_TRANS_Y {
        for x in 0..NUM_TRANS_X {
            let pos = Vector3::new(TRANS_SIZE * x as Float, TRANS_SIZE * y as Float, 0.);
            system.add_wave_source(T4010A1::new(pos, Vector3::z(), 1.0, 0.0, FREQUENCY));
        }
    }
    FocalPoint::new(focal_pos).optimize(&mut system);

    let r = 100.0;
    let area = GridAreaBuilder::new()
        .x_range(array_center[0] - r / 2.0, array_center[0] + r / 2.0)
        .y_range(array_center[1] - r / 2.0, array_center[1] + r / 2.0)
        .z_at(z)
        .resolution(1.)
        .generate();
    let mut field = PressureField::new();

    let calculator = CpuCalculator::new();
    calculator.calculate(&system, &area, &mut field);

    // Print to png image
    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("focus.png", field, bb);

    /////////////////////////////// Bessel Beam ///////////////////////////////

    BesselBeam::new(array_center, Vector3::z(), 18.0 / 180.0 * PI).optimize(&mut system);
    let area = GridAreaBuilder::new()
        .x_range(array_center[0] - r, array_center[0] + r)
        .z_range(0., 500.)
        .y_at(array_center[1])
        .resolution(1.)
        .generate();
    calculator.calculate(&system, &area, &mut field);

    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.z());
    write_image!("bessel.png", field, bb);
}
