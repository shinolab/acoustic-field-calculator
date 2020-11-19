/*
 * File: optimizer.rs
 * Project: src
 * Created Date: 22/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use libc::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::mem::forget;

use super::type_inference_aux::*;

use acoustic_field_calculator::prelude::*;

use acoustic_field_optimizer::multiple_foci::*;
use acoustic_field_optimizer::*;

macro_rules! gen_match_src_type {
    ([$($src_type:ident),*], $st: ident, $handle: ident, $expr: expr) => {
        match SourceType::from_i32($st) {
            $(SourceType::$src_type => {
                let mut system: Box<UniformSystem<$src_type>> = Box::from_raw($handle as *mut _);
                $expr.optimize(&mut system);
                forget(system);
            },)*
        }
    };
    ($st: ident, $handle: ident, $expr: expr) => {
      sources!(gen_match_src_type; $st, $handle, $expr)
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn AFO_FocalPoint(handle: *mut c_void, point: Vector3, source_type: i32) {
    gen_match_src_type!(source_type, handle, FocalPoint::new(point));
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn AFO_BesselBeam(
    handle: *mut c_void,
    point: Vector3,
    dir: Vector3,
    theta: f32,
    source_type: i32,
) {
    gen_match_src_type!(source_type, handle, BesselBeam::new(point, dir, theta));
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn AFO_IFFT(
    handle: *mut c_void,
    path: *const c_char,
    bottom_left: Vector3,
    top_left: Vector3,
    bottom_right: Vector3,
    spacing: f32,
    z: f32,
    source_type: i32,
) {
    let path = CStr::from_ptr(path).to_str().unwrap();
    gen_match_src_type!(
        source_type,
        handle,
        IFFT::new(path, bottom_left, top_left, bottom_right, spacing, z)
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_GSPAT(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    source_type: i32,
) {
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    gen_match_src_type!(
        source_type,
        handle,
        GSPAT::new(foci.to_vec(), amps.to_vec())
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_GS(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    source_type: i32,
) {
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    gen_match_src_type!(source_type, handle, GS::new(foci.to_vec(), amps.to_vec()));
}

#[no_mangle]
pub unsafe extern "C" fn AFO_Naive(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    source_type: i32,
) {
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    gen_match_src_type!(
        source_type,
        handle,
        Naive::new(foci.to_vec(), amps.to_vec())
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_Horn(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    source_type: i32,
) {
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    gen_match_src_type!(source_type, handle, Horn::new(foci.to_vec(), amps.to_vec()));
}

#[no_mangle]
pub unsafe extern "C" fn AFO_Long(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    source_type: i32,
) {
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    gen_match_src_type!(source_type, handle, Long::new(foci.to_vec(), amps.to_vec()));
}

#[no_mangle]
pub unsafe extern "C" fn AFO_APO(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    lambda: f32,
    source_type: i32,
) {
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    gen_match_src_type!(
        source_type,
        handle,
        APO::new(foci.to_vec(), amps.to_vec(), lambda)
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_GaussNewton(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    source_type: i32,
) {
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    gen_match_src_type!(
        source_type,
        handle,
        GaussNewton::new(foci.to_vec(), amps.to_vec())
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_GradientDescent(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    source_type: i32,
) {
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    gen_match_src_type!(
        source_type,
        handle,
        GradientDescent::new(foci.to_vec(), amps.to_vec())
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_LM(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    source_type: i32,
) {
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    gen_match_src_type!(source_type, handle, LM::new(foci.to_vec(), amps.to_vec()));
}
