/*
 * File: grid_field_builder.rs
 * Project: grid_field
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::bounds::Bounds;
use super::dimension::Axis;
use super::*;
use crate::core::Float;
use crate::Vector3;

use typenum::{N1, N2, N3};

use std::marker::PhantomData;
use std::mem::transmute;

pub struct Empty;
pub struct Unused;
pub struct Used;

/// Builder for GridArea
pub struct GridAreaBuilder<X, Y, Z, R> {
    dimension: Vec<Axis>,
    x_range: (Float, Float),
    y_range: (Float, Float),
    z_range: (Float, Float),
    resolution: Float,
    x_range_state: PhantomData<X>,
    y_range_state: PhantomData<Y>,
    z_range_state: PhantomData<Z>,
    resolution_state: PhantomData<R>,
}

/// **Note**: The observation points order changes according to the order in which the observation range/point of each axis is specified.
impl GridAreaBuilder<Empty, Empty, Empty, Empty> {
    pub fn new() -> Self {
        Self {
            dimension: vec![],
            x_range: (0., 0.),
            y_range: (0., 0.),
            z_range: (0., 0.),
            resolution: 0.,
            x_range_state: PhantomData,
            y_range_state: PhantomData,
            z_range_state: PhantomData,
            resolution_state: PhantomData,
        }
    }
}

impl<Y, Z, R> GridAreaBuilder<Empty, Y, Z, R> {
    /// Set observation range along x-axis
    ///
    /// # Arguments
    ///
    /// * `x_min` - Start point of the observation range
    /// * `x_max` - End point of the observation range
    pub fn x_range(mut self, x_min: Float, x_max: Float) -> GridAreaBuilder<Used, Y, Z, R> {
        self.x_range = (x_min, x_max);
        self.dimension.push(Axis::X);
        unsafe { transmute(self) }
    }

    /// Set observation point on x-axis
    ///
    /// # Arguments
    ///
    /// * `x` - observation point
    pub fn x_at(mut self, x: Float) -> GridAreaBuilder<Unused, Y, Z, R> {
        self.x_range = (x, x);
        unsafe { transmute(self) }
    }
}

impl<X, Z, R> GridAreaBuilder<X, Empty, Z, R> {
    /// Set observation range along y-axis
    ///
    /// # Arguments
    ///
    /// * `y_min` - Start point of the observation range
    /// * `y_max` - End point of the observation range
    pub fn y_range(mut self, y_min: Float, y_max: Float) -> GridAreaBuilder<X, Used, Z, R> {
        self.y_range = (y_min, y_max);
        self.dimension.push(Axis::Y);
        unsafe { transmute(self) }
    }

    /// Set observation point on y-axis
    ///
    /// # Arguments
    ///
    /// * `y` - observation point
    pub fn y_at(mut self, y: Float) -> GridAreaBuilder<X, Unused, Z, R> {
        self.y_range = (y, y);
        unsafe { transmute(self) }
    }
}

impl<X, Y, R> GridAreaBuilder<X, Y, Empty, R> {
    /// Set observation range along z-axis
    ///
    /// # Arguments
    ///
    /// * `z_min` - Start point of the observation range
    /// * `z_max` - End point of the observation range
    pub fn z_range(mut self, z_min: Float, z_max: Float) -> GridAreaBuilder<X, Y, Used, R> {
        self.z_range = (z_min, z_max);
        self.dimension.push(Axis::Z);
        unsafe { transmute(self) }
    }

    /// Set observation point on z-axis
    ///
    /// # Arguments
    ///
    /// * `z` - observation point
    pub fn z_at(mut self, z: Float) -> GridAreaBuilder<X, Y, Unused, R> {
        self.z_range = (z, z);
        unsafe { transmute(self) }
    }
}

impl<X, Y, Z> GridAreaBuilder<X, Y, Z, Empty> {
    /// Set resolution, that is spacing of the Grid
    ///
    /// # Arguments
    ///
    /// * `resolution` - resolution
    pub fn resolution(mut self, resolution: Float) -> GridAreaBuilder<X, Y, Z, Used> {
        self.resolution = resolution;
        unsafe { transmute(self) }
    }
}

macro_rules! impl_1d {
    ($X: ty , $Y: ty, $Z: ty) => {
        /// Generate one-dimensional GridArea.
        /// You must specify the observation range/point on all axes and resolution before
        impl GridAreaBuilder<$X, $Y, $Z, Used> {
            pub fn generate(&self) -> GridArea<N1> {
                let nx = ((self.x_range.1 - self.x_range.0) / self.resolution) as usize + 1;
                let ny = ((self.y_range.1 - self.y_range.0) / self.resolution) as usize + 1;
                let nz = ((self.z_range.1 - self.z_range.0) / self.resolution) as usize + 1;
                let origin = Vector3::new(self.x_range.0, self.y_range.0, self.z_range.0);
                GridArea::<N1>::new(
                    self.dimension[0],
                    Bounds::new(nx, ny, nz),
                    origin,
                    self.resolution,
                )
            }
        }
    };
}

macro_rules! impl_2d {
    ($X: ty , $Y: ty, $Z: ty) => {
        /// Generate two-dimensional GridArea.
        /// You must specify the observation range/point on all axes and resolution before
        impl GridAreaBuilder<$X, $Y, $Z, Used> {
            pub fn generate(&self) -> GridArea<N2> {
                let nx = ((self.x_range.1 - self.x_range.0) / self.resolution) as usize + 1;
                let ny = ((self.y_range.1 - self.y_range.0) / self.resolution) as usize + 1;
                let nz = ((self.z_range.1 - self.z_range.0) / self.resolution) as usize + 1;
                let origin = Vector3::new(self.x_range.0, self.y_range.0, self.z_range.0);
                GridArea::<N2>::new(
                    (self.dimension[0], self.dimension[1]),
                    Bounds::new(nx, ny, nz),
                    origin,
                    self.resolution,
                )
            }
        }
    };
}

impl_1d!(Used, Unused, Unused);
impl_1d!(Unused, Used, Unused);
impl_1d!(Unused, Unused, Used);

impl_2d!(Used, Used, Unused);
impl_2d!(Unused, Used, Used);
impl_2d!(Used, Unused, Used);

impl GridAreaBuilder<Used, Used, Used, Used> {
    /// Generate three-dimensional GridArea.
    /// You must specify the observation range/point on all axes and resolution before
    pub fn generate(self) -> GridArea<N3> {
        let nx = ((self.x_range.1 - self.x_range.0) / self.resolution) as usize + 1;
        let ny = ((self.y_range.1 - self.y_range.0) / self.resolution) as usize + 1;
        let nz = ((self.z_range.1 - self.z_range.0) / self.resolution) as usize + 1;
        let origin = Vector3::new(self.x_range.0, self.y_range.0, self.z_range.0);
        GridArea::<N3>::new(
            (self.dimension[0], self.dimension[1], self.dimension[2]),
            Bounds::new(nx, ny, nz),
            origin,
            self.resolution,
        )
    }
}

impl Default for GridAreaBuilder<Empty, Empty, Empty, Empty> {
    fn default() -> Self {
        Self::new()
    }
}
