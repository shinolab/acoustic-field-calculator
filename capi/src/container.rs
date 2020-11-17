/*
 * File: container.rs
 * Project: src
 * Created Date: 17/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::type_inference_aux::SourceType;
use acoustic_field_calculator::prelude::*;

use std::ffi::c_void;
use std::mem::forget;

#[no_mangle]
pub unsafe extern "C" fn AFC_CreateContainer(out: *mut *mut c_void, source_type: i32) {
    macro_rules! gen_container {
        ($($t:ident ),*) => {
            match SourceType::from_i32(source_type) {
                $(SourceType::$t => {
                    let container = WaveSourceContainer::<$t>::new();
                    let mut container = Box::new(container);
                    let ptr = container.as_mut() as *mut _;
                    forget(container);
                    *out = ptr as *mut _;
                },)*
            }
        }
    }
    gen_container!(SphereWaveSource, T4010A1);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn AFC_AddSphereWaveSource(handle: *mut c_void, source: SphereWaveSource) {
    let container: Box<WaveSourceContainer<SphereWaveSource>> = Box::from_raw(handle as *mut _);
    container.add_wave_source(source);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn AFC_AddT4010A1(handle: *mut c_void, source: T4010A1) {
    let container: Box<WaveSourceContainer<T4010A1>> = Box::from_raw(handle as *mut _);
    container.add_wave_source(source);
}

#[no_mangle]
pub unsafe extern "C" fn AFC_GetWaveSources(
    handle: *mut c_void,
    out: *mut *mut c_void,
    source_type: i32,
) -> u64 {
    let source_type = SourceType::from_i32(source_type);
    macro_rules! match_src_type {
        ($( $src_type:ident),*) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<WaveSourceContainer<$src_type>> = Box::from_raw(handle as *mut _);
                    let sources = (*calc).wave_sources_mut();
                    let ptr = sources.as_ptr() as *mut $src_type;
                    let len = sources.len();
                    forget(calc);
                    *out = ptr as *mut c_void;
                    len as u64
                },)*
            }
        }
    }
    match_src_type!(SphereWaveSource, T4010A1)
}
