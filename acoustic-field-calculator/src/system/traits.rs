/*
 * File: traits.rs
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

use crate::core::wave_sources::WaveSource;
use crate::core::{Complex, Vector3};

pub trait WaveSourceContainer<S: WaveSource> {
    /// Returns all of the wave sources
    fn wave_sources(&self) -> &[S];

    /// Returns all of the wave sources as mutable
    fn wave_sources_mut(&mut self) -> &mut Vec<S>;

    /// Add new wave source
    ///
    /// # Arguments
    ///
    /// * `source` - Wave source
    ///
    fn add_wave_source(&mut self, source: S);

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

pub trait PropagationMedium<S: WaveSource>: Sync + WaveSourceContainer<S> {
    fn propagate(&self, target: Vector3) -> Complex;
}
