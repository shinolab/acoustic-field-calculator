/*
 * File: calculator.rs
 * Project: src
 * Created Date: 09/05/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::type_inference_aux::*;
use acoustic_field_calculator::{accurate::*, calculator::*, gpu::*, prelude::*};

use std::ffi::c_void;
use std::mem::forget;

#[no_mangle]
pub unsafe extern "C" fn AFC_CreateCalculator(out: *mut *mut c_void, calc_type: i32) {
    macro_rules! gen_calc {
        ($($t:ident ),*) => {
            match CalculatorType::from_i32(calc_type) {
                $(CalculatorType::$t => {
                    let calc = $t::new();
                    let mut calc = Box::new(calc);
                    let ptr = calc.as_mut() as *mut _;
                    forget(calc);
                    *out = ptr as *mut _;
                },)*
            }
        }
    }
    calculators!(gen_calc);
}

#[no_mangle]
pub unsafe extern "C" fn AFC_FreeCalculator(handle: *mut c_void, calc_type: i32) {
    macro_rules! free_calc {
        ($($t:ident ),*) => {
            match CalculatorType::from_i32(calc_type) {
                $(CalculatorType::$t => {
                    let _calc: Box<$t> = Box::from_raw(handle as *mut _);
                },)*
            }
        }
    }
    calculators!(free_calc);
}

#[no_mangle]
pub unsafe extern "C" fn AFC_Calculate(
    calc_handle: *mut c_void,
    system_handle: *mut c_void,
    obs_handle: *mut c_void,
    field_handle: *mut c_void,
    out: *mut *const c_void,
    calc_type: i32,
    source_type: i32,
    obs_type: i32,
    field_type: i32,
) -> u64 {
    macro_rules! gen_match_field_type {
            ([$($ft:ident),*], $system: ident, $area: ident, $calc: ident) => {
                match FieldTypes::from_i32(field_type) {
                    $(FieldTypes::$ft => {
                        let mut field: Box<$ft> = Box::from_raw(field_handle as *mut _);
                        $calc.calculate(&*$system, &*$area, &mut *field);
                        let res = field.buffer();
                        let len = res.len();
                        let ptr = res.as_ptr() as *const _;
                        forget(field);
                        *out = ptr as *const c_void;
                        len as u64
                    },)*
                }
            }
    }
    macro_rules! gen_match_obs_type {
            ([$($ot:ident),*], $system: ident, $calc: ident) => {
                match ObserveAreaType::from_i32(obs_type) {
                    $(ObserveAreaType::$ot => {
                        let area: Box<$ot> = Box::from_raw(obs_handle as *mut _);
                        let len = fields!(gen_match_field_type; $system, area, $calc);
                        forget(area);
                        len
                    },)*
                }
            }
        }
    macro_rules! gen_match_src_type {
            ([$($st:ident),*], $calc: ident) => {
                match SourceType::from_i32(source_type) {
                    $(SourceType::$st => {
                        let system: Box<UniformSystem<$st>> = Box::from_raw(system_handle as *mut _);
                        let len =  obs_areas!(gen_match_obs_type; system, $calc);
                        forget(system);
                        len
                    },)*
                }
            }
        }
    macro_rules! match_calc_type {
            ($($ct:ident),*) => {
                match CalculatorType::from_i32(calc_type) {
                    $(CalculatorType::$ct => {
                        let calc: Box<$ct> = Box::from_raw(calc_handle as *mut _);
                        let len = sources!(gen_match_src_type; calc);
                        forget(calc);
                        len
                    },)*
                }
            }
    }
    calculators!(match_calc_type)
}
