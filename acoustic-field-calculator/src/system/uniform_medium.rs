/*
 * File: uniform_medium.rs
 * Project: system
 * Created Date: 18/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
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

pub trait UniformWaveSource: WaveSource {
    fn propagate(&self, target: Vector3, wavenum: Float, atten: Float) -> Complex;
}

impl UniformWaveSource for SphereWaveSource {
    fn propagate(&self, target: Vector3, wavenum: Float, atten: Float) -> Complex {
        let diff = crate::fmath::sub(target, self.position());
        let dist = diff.norm();
        let r = self.amp() * (-dist * atten).exp() / dist;
        let phi = self.phase() + wavenum * dist;
        Complex::from_polar(r, phi)
    }
}

impl UniformWaveSource for T4010A1 {
    fn propagate(&self, target: Vector3, wavenum: Float, atten: Float) -> Complex {
        let diff = crate::fmath::sub(target, self.position());
        let dist = diff.norm();
        let theta = crate::fmath::acos(self.direction().dot(&diff) / dist);
        let d = T4010A1::directivity(theta);
        let r = self.amp() * d * (-dist * atten).exp() / dist;
        let phi = self.phase() + wavenum * dist;
        Complex::from_polar(r, phi)
    }
}

pub struct UniformSystem<S: UniformWaveSource> {
    wave_sources: Vec<S>,
    wavenums: Vec<Float>,
    attens: Vec<Float>,
    temperature: Float,
}

impl<S: UniformWaveSource> UniformSystem<S> {
    pub fn new(temperature: Float) -> Self {
        Self {
            wave_sources: vec![],
            wavenums: vec![],
            attens: vec![],
            temperature,
        }
    }

    pub fn add_wave_source_with_wavenum(&mut self, source: S, wavenum: Float) {
        let atten = attenuation_coef(source.frequency(), 30., 1., 1., self.temperature);
        self.add_wave_source_with_wavenum_and_atten(source, wavenum, atten);
    }

    pub fn add_wave_source_with_atten(&mut self, source: S, atten: Float) {
        let sound_speed = calc_sound_speed(self.temperature);
        let wavenum = 2.0 * PI * source.frequency() / sound_speed;
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
}

impl<S: UniformWaveSource> WaveSourceContainer<S> for UniformSystem<S> {
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

impl<S: UniformWaveSource> PropagationMedium for UniformSystem<S> {
    fn propagate(&self, target: Vector3) -> Complex {
        self.wave_sources
            .iter()
            .zip(self.wavenums.iter())
            .zip(self.attens.iter())
            .map(|((source, &wavenum), &atten)| source.propagate(target, wavenum, atten))
            .sum()
    }
}
