/*
 * File: container.rs
 * Project: wave_sources
 * Created Date: 16/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::wave_sources::WaveSource;
pub struct WaveSourceContainer<S: WaveSource> {
    sources: Vec<S>,
}

impl<S: WaveSource> WaveSourceContainer<S> {
    pub fn new() -> Self {
        Self { sources: vec![] }
    }

    /// Returns all of the wave sources
    pub fn wave_sources(&self) -> &[S] {
        &self.sources
    }

    /// Returns all of the wave sources as mutable
    pub fn wave_sources_mut(&mut self) -> &mut Vec<S> {
        &mut self.sources
    }

    /// Add new wave source
    ///
    /// # Arguments
    ///
    /// * `source` - Wave source
    ///
    pub fn add_wave_source(&mut self, source: S) {
        self.sources.push(source);
    }
    /// Add new wave sources
    ///
    /// # Arguments
    ///
    /// * `sources` - A vector of wave sources
    ///
    pub fn add_wave_sources(&mut self, sources: Vec<S>) {
        for source in sources {
            self.add_wave_source(source)
        }
    }
}

impl<S: WaveSource> Default for WaveSourceContainer<S> {
    fn default() -> Self {
        Self::new()
    }
}
