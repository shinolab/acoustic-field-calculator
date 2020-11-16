/*
 * File: traits.rs
 * Project: calculator
 * Created Date: 19/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::core::wave_sources::WaveSource;
use crate::core::Float;

pub trait WaveSourceContainer<S: WaveSource> {
    /// Returns all of the wave sources
    fn wave_sources(&self) -> &[S];
    /// Returns all of the wave sources as mutable
    fn wave_sources_mut(&mut self) -> &mut Vec<S>;
    /// Returns speed of sound
    fn sound_speed(&self) -> Float;

    /// Add new wave source
    ///
    /// # Arguments
    ///
    /// * `source` - Wave source
    ///
    fn add_wave_source(&mut self, mut source: S) {
        source.set_sound_speed(self.sound_speed());
        self.wave_sources_mut().push(source);
    }
    /// Add new wave sources
    ///
    /// # Arguments
    ///
    /// * `sources` - A vector of wave sources
    ///
    fn add_wave_sources(&mut self, sources: Vec<S>) {
        for source in sources {
            self.add_wave_source(source)
        }
    }
}
