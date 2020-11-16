/*
 * File: optimizer.rs
 * Project: src
 * Created Date: 22/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/11/2020
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

use acoustic_field_calculator::accurate::*;
use acoustic_field_calculator::calculator::*;
use acoustic_field_calculator::gpu::*;
use acoustic_field_calculator::prelude::*;

use acoustic_field_optimizer::multiple_foci::*;
use acoustic_field_optimizer::*;

macro_rules! match_calc_type {
    ([$($ct:ident ),*], $calc_type: ident) => {
        match $calc_type {
            $(CalculatorType::$ct => {
                gen_match_src_type!([SphereWaveSource, T4010A1], $ct);
            },)*
        }
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn AFO_FocalPoint(
    handle: *mut c_void,
    point: Vector3,
    calc_type: i32,
    source_type: i32,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                  FocalPoint::new(point).optimize((*calc).wave_sources_mut());
                    forget(calc);

                },)*
            }
        }
    }
    match_calc_type!(
        [CpuCalculator, AccurateCalculator, GpuCalculator],
        calc_type
    );
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn AFO_BesselBeam(
    handle: *mut c_void,
    point: Vector3,
    dir: Vector3,
    theta: f32,
    calc_type: i32,
    source_type: i32,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                    BesselBeam::new(point, dir, theta).optimize((*calc).wave_sources_mut());
                    forget(calc);

                },)*
            }
        }
    }
    match_calc_type!(
        [CpuCalculator, AccurateCalculator, GpuCalculator],
        calc_type
    );
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
    calc_type: i32,
    source_type: i32,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    let path = CStr::from_ptr(path).to_str().unwrap();
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                    IFFT::new(path, bottom_left, top_left, bottom_right, spacing, z).optimize((*calc).wave_sources_mut());
                    forget(calc);

                },)*
            }
        }
    }
    match_calc_type!(
        [CpuCalculator, AccurateCalculator, GpuCalculator],
        calc_type
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_GSPAT(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    calc_type: i32,
    source_type: i32,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    let len = size as usize;
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
                    let amps = std::slice::from_raw_parts(amps, len);
                    GSPAT::new(foci.to_vec(),amps.to_vec()).optimize((*calc).wave_sources_mut());
                    forget(calc);

                },)*
            }
        }
    }
    match_calc_type!(
        [CpuCalculator, AccurateCalculator, GpuCalculator],
        calc_type
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_GS(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    calc_type: i32,
    source_type: i32,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    let len = size as usize;
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
                    let amps = std::slice::from_raw_parts(amps, len);
                    GS::new(foci.to_vec(),amps.to_vec()).optimize((*calc).wave_sources_mut());
                    forget(calc);

                },)*
            }
        }
    }
    match_calc_type!(
        [CpuCalculator, AccurateCalculator, GpuCalculator],
        calc_type
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_Naive(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    calc_type: i32,
    source_type: i32,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    let len = size as usize;
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
                    let amps = std::slice::from_raw_parts(amps, len);
                    Naive::new(foci.to_vec(),amps.to_vec()).optimize((*calc).wave_sources_mut());
                    forget(calc);

                },)*
            }
        }
    }
    match_calc_type!(
        [CpuCalculator, AccurateCalculator, GpuCalculator],
        calc_type
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_Horn(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    calc_type: i32,
    source_type: i32,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    let len = size as usize;
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
                    let amps = std::slice::from_raw_parts(amps, len);
                    Horn::new(foci.to_vec(),amps.to_vec()).optimize((*calc).wave_sources_mut());
                    forget(calc);

                },)*
            }
        }
    }
    match_calc_type!(
        [CpuCalculator, AccurateCalculator, GpuCalculator],
        calc_type
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_Long(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    calc_type: i32,
    source_type: i32,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    let len = size as usize;
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
                    let amps = std::slice::from_raw_parts(amps, len);
                    Long::new(foci.to_vec(),amps.to_vec()).optimize((*calc).wave_sources_mut());
                    forget(calc);

                },)*
            }
        }
    }
    match_calc_type!(
        [CpuCalculator, AccurateCalculator, GpuCalculator],
        calc_type
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_APO(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    lambda: f32,
    calc_type: i32,
    source_type: i32,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    let len = size as usize;
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
                    let amps = std::slice::from_raw_parts(amps, len);
                    APO::new(foci.to_vec(),amps.to_vec(), lambda).optimize((*calc).wave_sources_mut());
                    forget(calc);

                },)*
            }
        }
    }
    match_calc_type!(
        [CpuCalculator, AccurateCalculator, GpuCalculator],
        calc_type
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_GaussNewton(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    calc_type: i32,
    source_type: i32,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    let len = size as usize;
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
                    let amps = std::slice::from_raw_parts(amps, len);
                    GaussNewton::new(foci.to_vec(),amps.to_vec()).optimize((*calc).wave_sources_mut());
                    forget(calc);

                },)*
            }
        }
    }
    match_calc_type!(
        [CpuCalculator, AccurateCalculator, GpuCalculator],
        calc_type
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_GradientDescent(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    calc_type: i32,
    source_type: i32,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    let len = size as usize;
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
                    let amps = std::slice::from_raw_parts(amps, len);
                    GradientDescent::new(foci.to_vec(),amps.to_vec()).optimize((*calc).wave_sources_mut());
                    forget(calc);

                },)*
            }
        }
    }
    match_calc_type!(
        [CpuCalculator, AccurateCalculator, GpuCalculator],
        calc_type
    );
}

#[no_mangle]
pub unsafe extern "C" fn AFO_LM(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    calc_type: i32,
    source_type: i32,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    let len = size as usize;
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                    let foci = std::slice::from_raw_parts(foci as *const Vector3, len);
                    let amps = std::slice::from_raw_parts(amps, len);
                    LM::new(foci.to_vec(),amps.to_vec()).optimize((*calc).wave_sources_mut());
                    forget(calc);

                },)*
            }
        }
    }
    match_calc_type!(
        [CpuCalculator, AccurateCalculator, GpuCalculator],
        calc_type
    );
}
