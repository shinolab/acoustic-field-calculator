/*
 * File: main.rs
 * Project: examples
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use acoustic_field_calculator::calculator::FieldCalculator;
use acoustic_field_calculator::prelude::*;

const NUM_TRANS_X: usize = 18;
const NUM_TRANS_Y: usize = 14;
const TRANS_SIZE: Float = 10.18; // mm
const FREQUENCY: Float = 40e3; // Hz
const TEMPERATURE: Float = 300.0; // K

macro_rules! write_image {
    ($filename: tt, $buffer: ident, $bb: ident) => {{
        use image::png::PngEncoder;
        use image::ColorType;
        use scarlet::colormap::ListedColorMap;
        use std::fs::File;

        let colormap = ListedColorMap::magma();

        let output = File::create($filename).unwrap();
        let max = $buffer.max_result() as Float;
        let pixels: Vec<_> = $buffer
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

#[cfg(feature = "gpu")]
type Calculator = GpuCalculator;
#[cfg(all(not(feature = "gpu"), feature = "accurate"))]
type Calculator = AccurateCalculator;
#[cfg(all(not(feature = "gpu"), not(feature = "accurate")))]
type Calculator = CpuCalculator;

fn main() {
    let array_center = Vector3::new(
        TRANS_SIZE * (NUM_TRANS_X - 1) as Float / 2.0,
        TRANS_SIZE * (NUM_TRANS_Y - 1) as Float / 2.0,
        0.,
    );
    let z = 150.0;
    let focal_pos = array_center + z * Vector3::z();

    // UniformSystem is a uniform medium of sound
    let mut system = UniformSystem::new(TEMPERATURE);
    println!("{}", system.info());

    // Placing sound sources which produce focus at 'focal_pos'
    let sound_speed = system.sound_speed();
    let amp = 1.0;
    let dir = Vector3::z();
    for y in 0..NUM_TRANS_Y {
        for x in 0..NUM_TRANS_X {
            let pos = Vector3::new(TRANS_SIZE * x as Float, TRANS_SIZE * y as Float, 0.);
            let d = (pos - focal_pos).norm();
            let wavelength = sound_speed / FREQUENCY;
            let phase = (d % wavelength) / wavelength;
            let phase = -2.0 * PI * phase;
            system.add_wave_source(T4010A1::new(pos, dir, amp, phase, FREQUENCY));
        }
    }
    println!("{}", system.info_of_source(0));

    // Generating observe range and type
    let r = 100.0;
    let area = GridAreaBuilder::new()
        .x_range(array_center[0] - r / 2.0, array_center[0] + r / 2.0)
        .y_range(array_center[1] - r / 2.0, array_center[1] + r / 2.0)
        .z_at(z)
        .resolution(1.)
        .generate();
    let mut buffer = PressureField::new();

    // Calculation
    let calculator = Calculator::new();
    let start = std::time::Instant::now();
    calculator.calculate(&system, &area, &mut buffer);
    println!(
        "Elapsed: {} [ms]",
        start.elapsed().as_micros() as f64 / 1000.0
    );

    // Print to png image
    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy.png", buffer, bb);

    ////////////////////// Moving focus ///////////////////////////////
    let focal_pos = focal_pos + Vector3::new(20., 20., 0.);
    for wave_source in system.wave_sources_mut() {
        let d = (wave_source.position() - focal_pos).norm();
        let wavelength = sound_speed / FREQUENCY;
        let phase = (d % wavelength) / wavelength;
        let phase = -2.0 * PI * phase;
        wave_source.set_phase(phase);
    }

    let start = std::time::Instant::now();
    calculator.calculate(&system, &area, &mut buffer);
    println!(
        "Elapsed: {} [ms]",
        start.elapsed().as_micros() as f64 / 1000.0
    );

    let bounds = area.bounds();
    let bb = (bounds.x(), bounds.y());
    write_image!("xy2.png", buffer, bb);
}
