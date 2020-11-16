/*
 * File: main.rs
 * Project: examples
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use acoustic_field_calculator::prelude::*;
use acoustic_field_calculator::Float;
use acoustic_field_calculator::*;

const NUM_TRANS_X: usize = 18;
const NUM_TRANS_Y: usize = 14;
const TRANS_SIZE: Float = 10.18;
const FREQUENCY: Float = 40e3;
const SOUND_SPEED: Float = 340e3;
const WAVE_LENGTH: Float = SOUND_SPEED / FREQUENCY;

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

    let mut calculator = {
        #[cfg(not(feature = "gpu"))]
        {
            #[cfg(feature = "accurate")]
            {
                println!("Accurate mode");
                CalculatorBuilder::new()
                    .set_sound_speed(SOUND_SPEED)
                    .set_accurate()
                    .generate()
            }
            #[cfg(not(feature = "accurate"))]
            {
                println!("Normal mode");
                CalculatorBuilder::new()
                    .set_sound_speed(SOUND_SPEED)
                    .generate()
            }
        }
        #[cfg(feature = "gpu")]
        {
            println!("GPU mode");
            CalculatorBuilder::new()
                .set_sound_speed(SOUND_SPEED)
                .gpu_enable()
                .generate()
        }
    };

    let amp = 1.0;
    for y in 0..NUM_TRANS_Y {
        for x in 0..NUM_TRANS_X {
            let pos = Vector3::new(TRANS_SIZE * x as Float, TRANS_SIZE * y as Float, 0.);
            let d = (pos - focal_pos).norm();
            let phase = (d % WAVE_LENGTH) / WAVE_LENGTH;
            let phase = -2.0 * PI * phase;
            calculator.add_wave_source(T4010A1::new(pos, Vector3::z(), amp, phase, FREQUENCY));
        }
    }

    let r = 100.0;
    let area = GridAreaBuilder::new()
        .x_range(array_center[0] - r / 2.0, array_center[0] + r / 2.0)
        .y_range(array_center[1] - r / 2.0, array_center[1] + r / 2.0)
        .z_at(z)
        .resolution(1.)
        .generate();

    let mut field = PressureField::new();

    let start = std::time::Instant::now();
    calculator.calculate(&area, &mut field);
    println!(
        "Elapsed: {} [ms]",
        start.elapsed().as_micros() as f64 / 1000.0
    );

    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy.png", field, bb);

    /////////////////////////////////////////////////////////////////////
    let focal_pos = focal_pos + Vector3::new(20., 20., 0.);
    for source in calculator.wave_sources_mut() {
        let d = (source.pos - focal_pos).norm();
        let phase = (d % WAVE_LENGTH) / WAVE_LENGTH;
        let phase = -2.0 * PI * phase;
        source.phase = phase;
    }

    let start = std::time::Instant::now();
    calculator.calculate(&area, &mut field);
    println!(
        "Elapsed: {} [ms]",
        start.elapsed().as_micros() as f64 / 1000.0
    );

    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy2.png", field, bb);
}