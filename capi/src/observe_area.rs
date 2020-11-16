/*
 * File: observe_area.rs
 * Project: src
 * Created Date: 21/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use acoustic_field_calculator::gpu::*;
use acoustic_field_calculator::observe_area::grid::*;
use acoustic_field_calculator::prelude::*;

use std::ffi::c_void;
use std::mem::forget;

#[no_mangle]
pub unsafe extern "C" fn AFC_CreateScatterArea(out: *mut *mut c_void) {
    let area = ScatterArea::new();
    let mut area = Box::new(area);
    let ptr = area.as_mut() as *mut _;
    forget(area);
    *out = ptr as *mut _;
}

#[no_mangle]
pub unsafe extern "C" fn AFC_FreeScatterArea(handle: *mut c_void) {
    let _area: Box<ScatterArea> = Box::from_raw(handle as *mut _);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn AFC_ScatterAddObservePoint(handle: *mut c_void, p: Vector3) {
    let mut area: Box<ScatterArea> = Box::from_raw(handle as *mut _);
    area.add_observe_point(p);
    forget(area);
}

#[no_mangle]
pub unsafe extern "C" fn AFC_CreateGridArea1D(
    out: *mut *mut c_void,
    first_start: f32,
    first_end: f32,
    second: f32,
    third: f32,
    resolution: f32,
    first_axis: i32,
    second_axis: i32,
    third_axis: i32,
) {
    let first_axis = Axis::from_i32(first_axis);
    let second_axis = Axis::from_i32(second_axis);
    let third_axis = Axis::from_i32(third_axis);
    let builder = GridAreaBuilder::new();
    macro_rules! gen_match {
        ($([($f:ident, $fg:ident),($s:ident, $sg:ident), ($t:ident, $tg:ident)]),*) => {
            match (first_axis, second_axis, third_axis) {
                $((Axis::$f, Axis::$s, Axis::$t) =>
                    builder
                    .$fg(first_start, first_end)
                    .$sg(second)
                    .$tg(third)
                    .resolution(resolution)
                    .generate()
                ,)*
                _ => unreachable!()
            }
        }
    }
    let area = gen_match!(
        [(X, x_range), (Y, y_at), (Z, z_at)],
        [(X, x_range), (Z, z_at), (Y, y_at)],
        [(Y, y_range), (Z, z_at), (X, x_at)],
        [(Y, y_range), (X, x_at), (Z, z_at)],
        [(Z, z_range), (X, x_at), (Y, y_at)],
        [(Z, z_range), (Y, y_at), (X, x_at)]
    );
    let mut area = Box::new(area);
    let ptr = area.as_mut() as *mut _;
    forget(area);
    *out = ptr as *mut _;
}

#[no_mangle]
pub unsafe extern "C" fn AFC_CreateGridArea2D(
    out: *mut *mut c_void,
    first_start: f32,
    first_end: f32,
    second_start: f32,
    second_end: f32,
    third: f32,
    resolution: f32,
    first_axis: i32,
    second_axis: i32,
    third_axis: i32,
) {
    let first_axis = Axis::from_i32(first_axis);
    let second_axis = Axis::from_i32(second_axis);
    let third_axis = Axis::from_i32(third_axis);
    let builder = GridAreaBuilder::new();
    macro_rules! gen_match {
        ($([($f:ident, $fg:ident),($s:ident, $sg:ident), ($t:ident, $tg:ident)]),*) => {
            match (first_axis, second_axis, third_axis) {
                $((Axis::$f, Axis::$s, Axis::$t) =>
                    builder
                    .$fg(first_start, first_end)
                    .$sg(second_start, second_end)
                    .$tg(third)
                    .resolution(resolution)
                    .generate()
                ,)*
                _ => unreachable!()
            }
        }
    }
    let area = gen_match!(
        [(X, x_range), (Y, y_range), (Z, z_at)],
        [(X, x_range), (Z, z_range), (Y, y_at)],
        [(Y, y_range), (Z, z_range), (X, x_at)],
        [(Y, y_range), (X, x_range), (Z, z_at)],
        [(Z, z_range), (X, x_range), (Y, y_at)],
        [(Z, z_range), (Y, y_range), (X, x_at)]
    );
    let mut area = Box::new(area);
    let ptr = area.as_mut() as *mut _;
    forget(area);
    *out = ptr as *mut _;
}

#[no_mangle]
pub unsafe extern "C" fn AFC_CreateGridArea3D(
    out: *mut *mut c_void,
    first_start: f32,
    first_end: f32,
    second_start: f32,
    second_end: f32,
    third_start: f32,
    third_end: f32,
    resolution: f32,
    first_axis: i32,
    second_axis: i32,
    third_axis: i32,
) {
    let first_axis = Axis::from_i32(first_axis);
    let second_axis = Axis::from_i32(second_axis);
    let third_axis = Axis::from_i32(third_axis);
    let builder = GridAreaBuilder::new();
    macro_rules! gen_match {
        ($([($f:ident, $fg:ident),($s:ident, $sg:ident), ($t:ident, $tg:ident)]),*) => {
            match (first_axis, second_axis, third_axis) {
                $((Axis::$f, Axis::$s, Axis::$t) =>
                    builder
                    .$fg(first_start, first_end)
                    .$sg(second_start, second_end)
                    .$tg(third_start, third_end)
                    .resolution(resolution)
                    .generate()
                ,)*
                _ => unreachable!()
            }
        }
    }
    let area = gen_match!(
        [(X, x_range), (Y, y_range), (Z, z_range)],
        [(X, x_range), (Z, z_range), (Y, y_range)],
        [(Y, y_range), (Z, z_range), (X, x_range)],
        [(Y, y_range), (X, x_range), (Z, z_range)],
        [(Z, z_range), (X, x_range), (Y, y_range)],
        [(Z, z_range), (Y, y_range), (X, x_range)]
    );
    let mut area = Box::new(area);
    let ptr = area.as_mut() as *mut _;
    forget(area);
    *out = ptr as *mut _;
}

#[no_mangle]
pub unsafe extern "C" fn AFC_FreeGridArea(handle: *mut c_void, dimension: i32) {
    match dimension {
        1 => {
            let _area: Box<GridArea<N1>> = Box::from_raw(handle as *mut _);
        }
        2 => {
            let _area: Box<GridArea<N2>> = Box::from_raw(handle as *mut _);
        }
        3 => {
            let _area: Box<GridArea<N3>> = Box::from_raw(handle as *mut _);
        }
        _ => unreachable!(),
    };
}

#[no_mangle]
pub unsafe extern "C" fn AFC_GetGridSize(
    handle: *mut c_void,
    dimension: i32,
    first: *mut u32,
    second: *mut u32,
    third: *mut u32,
) {
    let bb = match dimension {
        1 => {
            let area: Box<GridArea<N1>> = Box::from_raw(handle as *mut _);
            let bb = area.size();
            forget(area);
            bb
        }
        2 => {
            let area: Box<GridArea<N2>> = Box::from_raw(handle as *mut _);
            let bb = area.size();
            forget(area);
            bb
        }
        3 => {
            let area: Box<GridArea<N3>> = Box::from_raw(handle as *mut _);
            let bb = area.size();
            forget(area);
            bb
        }
        _ => unreachable!(),
    };
    *first = bb.0;
    *second = bb.1;
    *third = bb.2;
}
