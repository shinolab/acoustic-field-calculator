/*
 * File: mod.rs
 * Project: scatter
 * Created Date: 20/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::observe_area::traits::*;
use crate::Vector3;

/// Scatter observation points
pub struct ScatterArea {
    observe_points: Vec<Vector3>,
}

impl ScatterArea {
    pub fn new() -> Self {
        Self {
            observe_points: vec![],
        }
    }

    /// Add a new observation point
    ///
    /// # Arguments
    ///
    /// * `v` - Observation point
    pub fn add_observe_point(&mut self, v: Vector3) {
        self.observe_points.push(v);
    }
}

impl Default for ScatterArea {
    fn default() -> Self {
        Self::new()
    }
}

impl ObserveArea for ScatterArea {
    fn observe_points(&self) -> &Vec<Vector3> {
        &self.observe_points
    }
}
