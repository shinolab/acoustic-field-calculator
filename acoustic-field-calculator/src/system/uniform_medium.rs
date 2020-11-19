/*
 * File: uniform_medium.rs
 * Project: system
 * Created Date: 18/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::{PropagationMedium, WaveSourceContainer};
use crate::core::{
    attenuation::attenuation_coef, sound_speed::calc_sound_speed, wave_sources::*, Complex, Float,
    Vector3, PI,
};
use num::Zero;

pub struct UniformSystem<S: WaveSource> {
    wave_sources: Vec<S>,
    wavenums: Vec<Float>,
    attens: Vec<Float>,
    temperature: Float,
    sound_speed: Float,
}

impl<S: WaveSource> UniformSystem<S> {
    pub fn new(temperature: Float) -> Self {
        Self {
            wave_sources: vec![],
            wavenums: vec![],
            attens: vec![],
            temperature,
            sound_speed: calc_sound_speed(temperature),
        }
    }

    pub fn add_wave_source_with_wavenum(&mut self, source: S, wavenum: Float) {
        let atten = attenuation_coef(source.frequency(), 30., 1., 1., self.temperature);
        self.add_wave_source_with_wavenum_and_atten(source, wavenum, atten);
    }

    pub fn add_wave_source_with_atten(&mut self, source: S, atten: Float) {
        let wavenum = 2.0 * PI * source.frequency() / self.sound_speed;
        self.add_wave_source_with_wavenum_and_atten(source, wavenum, atten);
    }

    pub fn add_wave_source_with_wavenum_and_atten(
        &mut self,
        source: S,
        wavenum: Float,
        atten: Float,
    ) {
        self.wave_sources.push(source);
        self.wavenums.push(wavenum);
        self.attens.push(atten);
    }

    pub fn wavenums(&self) -> &[Float] {
        &self.wavenums
    }

    pub fn attens(&self) -> &[Float] {
        &self.attens
    }

    pub fn sound_speed(&self) -> Float {
        self.sound_speed
    }

    pub fn info(&self) -> String {
        format!(
            "Uniform Medium:\n Temperature: {} K = {} ℃\n Sound Speed: {} mm/s",
            self.temperature,
            self.temperature - 273.15,
            calc_sound_speed(self.temperature)
        )
    }

    pub fn info_of_source(&self, idx: usize) -> String {
        format!(
            "{}-th wave source:\n Wavelength: {} mm\n Wavenumber: {} mm^-1\n Attenuation: {} Np/mm",
            idx,
            2.0 * PI / self.wavenums[idx],
            self.wavenums[idx],
            self.attens[idx]
        )
    }
}

impl<S: WaveSource> WaveSourceContainer<S> for UniformSystem<S> {
    fn wave_sources(&self) -> &[S] {
        &self.wave_sources
    }

    fn wave_sources_mut(&mut self) -> &mut Vec<S> {
        &mut self.wave_sources
    }

    fn add_wave_source(&mut self, source: S) {
        let sound_speed = calc_sound_speed(self.temperature);
        let wavenum = 2.0 * PI * source.frequency() / sound_speed;
        let atten = attenuation_coef(source.frequency(), 30., 1., 1., self.temperature);
        self.add_wave_source_with_wavenum_and_atten(source, wavenum, atten);
    }
}

impl<S: WaveSource> PropagationMedium<S> for UniformSystem<S> {
    fn propagate(&self, target: Vector3) -> Complex {
        let sources = self.wave_sources();
        let wavenums = self.wavenums();
        let attens = self.attens();
        let mut c = Complex::zero();
        for i in 0..sources.len() {
            let source = &sources[i];
            let diff = crate::fmath::sub(target, source.position());
            let dist = diff.norm();
            let theta = crate::fmath::acos(source.direction().dot(&diff) / dist);
            let d = S::directivity(theta);
            let r = source.amp() * d * (-dist * attens[i]).exp() / dist;
            let phi = source.phase() + wavenums[i] * dist;
            c += Complex::from_polar(r, phi);
        }
        c
    }
}
