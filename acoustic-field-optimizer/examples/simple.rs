/*
 * File: simple.rs
 * Project: examples
 * Created Date: 27/05/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use acoustic_field_calculator::prelude::*;
use acoustic_field_optimizer::*;

use acoustic_field_optimizer::PI;

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

    let calculator = CpuCalculator::new();

    let mut container = WaveSourceContainer::new();
    let amp = 1.0;
    for y in 0..NUM_TRANS_Y {
        for x in 0..NUM_TRANS_X {
            let pos = Vector3::new(TRANS_SIZE * x as Float, TRANS_SIZE * y as Float, 0.);
            container.add_wave_source(T4010A1::new(pos, Vector3::z(), amp, 0.0, FREQUENCY));
        }
    }

    FocalPoint::new(focal_pos, SOUND_SPEED).optimize(container.wave_sources_mut());

    let r = 100.0;
    let area = GridAreaBuilder::new()
        .x_range(array_center[0] - r, array_center[0] + r)
        .y_range(array_center[1] - r, array_center[1] + r)
        .z_at(z)
        .resolution(1.)
        .generate();

    let mut field = FieldBuilder::new()
        .pressure()
        .sound_speed(SOUND_SPEED)
        .build();

    calculator.calculate(&mut container, &area, &mut field);

    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy_focus.png", field, bb);

    /////////////////////////////// Bessel Beam ///////////////////////////////
    BesselBeam::new(array_center, Vector3::z(), 18.0 / 180.0 * PI, SOUND_SPEED)
        .optimize(container.wave_sources_mut());

    let buffer = GridAreaBuilder::new()
        .x_range(array_center[0] - r, array_center[0] + r)
        .z_range(0., 500.)
        .y_at(array_center[1])
        .resolution(1.)
        .generate();

    calculator.calculate(&mut container, &buffer, &mut field);

    let bounds = buffer.bounds();
    let bb = (bounds.x(), bounds.z());
    write_image!("xz_bessel.png", field, bb);
}
