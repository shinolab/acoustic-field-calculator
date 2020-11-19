/*
 * File: system.rs
 * Project: src
 * Created Date: 17/11/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::type_inference_aux::SourceType;
use acoustic_field_calculator::prelude::*;

use std::ffi::{c_void, CString};
use std::mem::forget;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn AFC_CreateUniformSystem(
    out: *mut *mut c_void,
    temperature: f32,
    source_type: i32,
) {
    macro_rules! gen_system {
        ($($t:ident),*) => {
            match SourceType::from_i32(source_type) {
                $(SourceType::$t => {
                    let system = UniformSystem::<$t>::new(temperature);
                    let mut system = Box::new(system);
                    let ptr = system.as_mut() as *mut _;
                    forget(system);
                    *out = ptr as *mut _;
                },)*
            }
        }
    }
    sources!(gen_system);
}

#[no_mangle]
pub unsafe extern "C" fn AFC_FreeUniformSystem(handle: *mut c_void, source_type: i32) {
    macro_rules! gen_system {
        ($($t:ident ),*) => {
            match SourceType::from_i32(source_type) {
                $(SourceType::$t => {
                    let _system: Box<UniformSystem<$t>> = Box::from_raw(handle as *mut _);
                },)*
            }
        }
    }
    sources!(gen_system);
}

#[no_mangle]
pub unsafe extern "C" fn AFC_UniformSystemSoundSpeed(handle: *mut c_void, source_type: i32) -> f32 {
    macro_rules! gen_system {
        ($($t:ident ),*) => {
            match SourceType::from_i32(source_type) {
                $(SourceType::$t => {
                    let system: Box<UniformSystem<$t>> = Box::from_raw(handle as *mut _);
                    let sound_speed = system.sound_speed();
                    forget(system);
                    sound_speed
                },)*
            }
        }
    }
    sources!(gen_system)
}

#[no_mangle]
pub unsafe extern "C" fn AFC_UniformSystemInfo(
    handle: *mut c_void,
    source_type: i32,
) -> *mut c_char {
    macro_rules! gen_system {
        ($($t:ident ),*) => {
            match SourceType::from_i32(source_type) {
                $(SourceType::$t => {
                    let system: Box<UniformSystem<$t>> = Box::from_raw(handle as *mut _);
                    let info = system.info();
                    forget(system);
                    CString::new(info).unwrap().into_raw()
                },)*
            }
        }
    }
    sources!(gen_system)
}

#[no_mangle]
pub unsafe extern "C" fn AFC_UniformSystemSourceInfo(
    handle: *mut c_void,
    source_idx: u64,
    source_type: i32,
) -> *mut c_char {
    macro_rules! gen_system {
        ($($t:ident ),*) => {
            match SourceType::from_i32(source_type) {
                $(SourceType::$t => {
                    let system: Box<UniformSystem<$t>> = Box::from_raw(handle as *mut _);
                    let info = system.info_of_source(source_idx as usize);
                    forget(system);
                    CString::new(info).unwrap().into_raw()
                },)*
            }
        }
    }
    sources!(gen_system)
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn AFC_AddSphereWaveSource(handle: *mut c_void, source: SphereWaveSource) {
    let mut system: Box<UniformSystem<SphereWaveSource>> = Box::from_raw(handle as *mut _);
    system.add_wave_source(source);
    forget(system);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn AFC_AddT4010A1(handle: *mut c_void, source: T4010A1) {
    let mut system: Box<UniformSystem<T4010A1>> = Box::from_raw(handle as *mut _);
    system.add_wave_source(source);
    forget(system);
}

#[no_mangle]
pub unsafe extern "C" fn AFC_GetWaveSources(
    handle: *mut c_void,
    out: *mut *mut c_void,
    source_type: i32,
) -> u64 {
    macro_rules! match_src_type {
        ($( $src_type:ident),*) => {
            match SourceType::from_i32(source_type) {
                $(SourceType::$src_type => {
                    let mut system: Box<UniformSystem<$src_type>> = Box::from_raw(handle as *mut _);
                    let sources = (*system).wave_sources_mut();
                    let ptr = sources.as_ptr() as *mut $src_type;
                    let len = sources.len();
                    forget(system);
                    *out = ptr as *mut c_void;
                    len as u64
                },)*
            }
        }
    }
    sources!(match_src_type)
}
