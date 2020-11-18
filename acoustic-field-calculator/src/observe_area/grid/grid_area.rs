/*
 * File: grid_field_1d.rs
 * Project: buffer
 * Created Date: 05/05/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::{
    bounds::Bounds,
    dimension::{Axis, Dimension},
    *,
};
use crate::{
    core::{Float, Vector3},
    field_type::*,
    observe_area::traits::*,
};

use std::marker::PhantomData;

/// Observation points on grid
pub struct GridArea<D, F: FieldType> {
    dimension: Dimension,
    bounds: Bounds,
    observe_points: Vec<Vector3>,
    results: Vec<F::Output>,
    _dim_num: PhantomData<D>,
}

impl<D, F: FieldType> GridArea<D, F> {
    fn new_impl<N>(
        dimension: Dimension,
        bounds: Bounds,
        observe_points: Vec<Vector3>,
    ) -> GridArea<N, F> {
        GridArea {
            dimension,
            bounds,
            observe_points,
            results: vec![],
            _dim_num: PhantomData,
        }
    }

    /// Returns a dimension
    pub fn dimension(&self) -> Dimension {
        self.dimension
    }

    ///  Returns a bounds
    pub fn bounds(&self) -> Bounds {
        self.bounds
    }
}

impl<F: FieldType> GridArea<N1, F> {
    pub(crate) fn new(axis: Axis, bounds: Bounds, origin: Vector3, resolution: Float) -> Self {
        Self::new_impl(
            Dimension::One(axis),
            bounds,
            Self::generate_observe_points(bounds, origin, resolution, axis),
        )
    }

    fn generate_observe_points(
        bounds: Bounds,
        origin: Vector3,
        resolution: Float,
        axis: Axis,
    ) -> Vec<Vector3> {
        let len = bounds.size();
        let iter: Box<dyn Iterator<Item = Vector3>> = match axis {
            Axis::X => Box::new((0..len).map(move |n| {
                Vector3::new(origin[0] + (n as Float * resolution), origin[1], origin[2])
            })),
            Axis::Y => Box::new((0..len).map(move |n| {
                Vector3::new(origin[0], origin[1] + (n as Float * resolution), origin[2])
            })),
            Axis::Z => Box::new((0..len).map(move |n| {
                Vector3::new(origin[0], origin[1], origin[2] + (n as Float * resolution))
            })),
            _ => unreachable!(),
        };
        iter.collect()
    }
}

impl<F: FieldType> GridArea<N2, F> {
    pub(crate) fn new(
        dim: (Axis, Axis),
        bounds: Bounds,
        origin: Vector3,
        resolution: Float,
    ) -> Self {
        Self::new_impl(
            Dimension::Two(dim.0, dim.1),
            bounds,
            Self::generate_observe_points(bounds, origin, resolution, dim),
        )
    }

    fn generate_observe_points(
        bounds: Bounds,
        origin: Vector3,
        resolution: Float,
        dim: (Axis, Axis),
    ) -> Vec<Vector3> {
        macro_rules! to_variable {
            (0, $x: ident, $y: ident, $z: ident) => {
                $x
            };
            (1, $x: ident, $y: ident, $z: ident) => {
                $y
            };
            (2, $x: ident, $y: ident, $z: ident) => {
                $z
            };
        }
        macro_rules! iter_gen {
            ($first:tt, $second:tt, $x: ident, $y: ident, $z: ident, $another: stmt , $r: ident, $b: ident, $o: ident) => {
                Box::new({
                    $another
                    iproduct!(
                        (0..$b[$second]).map(move |n| $o[$second] + (n as Float * $r)),
                        (0..$b[$first]).map(move |n| $o[$first] + (n as Float * $r))
                    )
                    .map(
                        move |(to_variable!($second, $x, $y, $z), to_variable!($first, $x, $y, $z))| {
                            Vector3::new($x, $y, $z)
                        },
                    )},
                )
            };
        }
        let resolution = resolution;
        let bounds = bounds;
        let origin = origin;
        let iter: Box<dyn Iterator<Item = Vector3>> = match dim {
            (Axis::X, Axis::Y) => {
                iter_gen!(0, 1, x, y, z, let z = origin[2], resolution, bounds, origin)
            }
            (Axis::Y, Axis::X) => {
                iter_gen!(1, 0, x, y, z, let z = origin[2], resolution, bounds, origin)
            }
            (Axis::Y, Axis::Z) => {
                iter_gen!(1, 2, x, y, z, let x = origin[0], resolution, bounds, origin)
            }
            (Axis::Z, Axis::Y) => {
                iter_gen!(2, 1, x, y, z, let x = origin[0], resolution, bounds, origin)
            }
            (Axis::Z, Axis::X) => {
                iter_gen!(2, 0, x, y, z, let y = origin[1], resolution, bounds, origin)
            }
            (Axis::X, Axis::Z) => {
                iter_gen!(0, 2, x, y, z, let y = origin[1], resolution, bounds, origin)
            }
            _ => unreachable!(),
        };
        iter.collect()
    }
}

impl<F: FieldType> GridArea<N3, F> {
    pub(crate) fn new(
        dim: (Axis, Axis, Axis),
        bounds: Bounds,
        origin: Vector3,
        resolution: Float,
    ) -> Self {
        Self::new_impl(
            Dimension::Three(dim.0, dim.1, dim.2),
            bounds,
            Self::generate_observe_points(bounds, origin, resolution, dim),
        )
    }

    fn generate_observe_points(
        bounds: Bounds,
        origin: Vector3,
        resolution: Float,
        dim: (Axis, Axis, Axis),
    ) -> Vec<Vector3> {
        macro_rules! to_variable {
            (0, $x: ident, $y: ident, $z: ident) => {
                $x
            };
            (1, $x: ident, $y: ident, $z: ident) => {
                $y
            };
            (2, $x: ident, $y: ident, $z: ident) => {
                $z
            };
        }
        macro_rules! iter_gen {
            ($first:tt, $second:tt, $third:tt, $x: ident, $y: ident, $z: ident, $r: ident, $b: ident, $o: ident) => {
                Box::new(
                    iproduct!(
                        (0..$b[$third]).map(move |n| $o[$third] + (n as Float * $r)),
                        (0..$b[$second]).map(move |n| $o[$second] + (n as Float * $r)),
                        (0..$b[$first]).map(move |n| $o[$first] + (n as Float * $r))
                    )
                    .map(
                        move |(
                            to_variable!($third, $x, $y, $z),
                            to_variable!($second, $x, $y, $z),
                            to_variable!($first, $x, $y, $z),
                        )| { Vector3::new($x, $y, $z) },
                    ),
                )
            };
        }
        let resolution = resolution;
        let bounds = bounds;
        let origin = origin;
        let iter: Box<dyn Iterator<Item = Vector3>> = match dim {
            (Axis::X, Axis::Y, Axis::Z) => iter_gen!(0, 1, 2, x, y, z, resolution, bounds, origin),
            (Axis::Z, Axis::X, Axis::Y) => iter_gen!(2, 0, 1, x, y, z, resolution, bounds, origin),
            (Axis::Y, Axis::Z, Axis::X) => iter_gen!(1, 2, 0, x, y, z, resolution, bounds, origin),
            (Axis::X, Axis::Z, Axis::Y) => iter_gen!(0, 2, 1, x, y, z, resolution, bounds, origin),
            (Axis::Y, Axis::X, Axis::Z) => iter_gen!(1, 0, 2, x, y, z, resolution, bounds, origin),
            (Axis::Z, Axis::Y, Axis::X) => iter_gen!(2, 1, 0, x, y, z, resolution, bounds, origin),
            _ => unreachable!(),
        };
        iter.collect()
    }
}

impl<D, F: FieldType> ObserveArea<F> for GridArea<D, F> {
    fn points_and_results_mut(&mut self) -> (&Vec<Vector3>, &mut Vec<F::Output>) {
        (&self.observe_points, &mut self.results)
    }

    fn results(&self) -> &[F::Output] {
        &self.results
    }

    fn points(&self) -> &[Vector3] {
        &self.observe_points
    }

    fn results_mut(&mut self) -> &mut Vec<F::Output> {
        &mut self.results
    }
}
