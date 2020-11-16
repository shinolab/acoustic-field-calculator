/*
 * File: bounds.rs
 * Project: buffer
 * Created Date: 08/05/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use std::ops::Index;

#[derive(Debug, Copy, Clone)]
/// Observation area size container
pub struct Bounds {
    x: usize,
    y: usize,
    z: usize,
}

impl Bounds {
    pub(crate) fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    /// Returns size along x-axis
    pub fn x(&self) -> usize {
        self.x
    }

    /// Returns size along y-axis
    pub fn y(&self) -> usize {
        self.y
    }

    /// Returns size along z-axis
    pub fn z(&self) -> usize {
        self.z
    }

    /// Returns total size
    pub fn size(&self) -> usize {
        self.x * self.y * self.z
    }
}

impl Index<usize> for Bounds {
    type Output = usize;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index is out of bounds."),
        }
    }
}
