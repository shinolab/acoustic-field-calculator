/*
 * File: calculator.rs
 * Project: src
 * Created Date: 09/05/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/10/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::type_inference_aux::*;
use acoustic_field_calculator::accurate::*;
use acoustic_field_calculator::calculator::*;
use acoustic_field_calculator::gpu::*;
use acoustic_field_calculator::observe_area::grid::*;
use acoustic_field_calculator::prelude::*;

use std::ffi::c_void;
use std::mem::forget;

#[no_mangle]
pub unsafe extern "C" fn AFC_CreateCalculator(
    out: *mut *mut c_void,
    c: f32,
    calc_type: i32,
    source_type: i32,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $builder: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let calc = $builder.generate::<$src_type>();
                    let mut calc = Box::new(calc);
                    let ptr = calc.as_mut() as *mut _;
                    forget(calc);
                    *out = ptr as *mut _;
                },)*
            }
        }
    }
    macro_rules! match_calc_type {
        ([$( $calc_type:ident => $initialize:expr ),*]) => {
            match calc_type {
                $(CalculatorType::$calc_type => {
                    let builder = $initialize;
                    gen_match_src_type!([SphereWaveSource, T4010A1], builder);
                },)*
            }
        }
    }
    match_calc_type!([
        CpuCalculator => CalculatorBuilder::new().set_sound_speed(c),
        AccurateCalculator => CalculatorBuilder::new().set_sound_speed(c).set_accurate(),
        GpuCalculator => CalculatorBuilder::new().set_sound_speed(c).gpu_enable()
    ]);
}

#[no_mangle]
pub unsafe extern "C" fn AFC_FreeCalculator(handle: *mut c_void, calc_type: i32, source_type: i32) {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let _calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                },)*
            }
        }
    }
    macro_rules! match_calc_type {
        ([$($ct:ident ),*]) => {
            match calc_type {
                $(CalculatorType::$ct => {
                    gen_match_src_type!([SphereWaveSource, T4010A1], $ct);
                },)*
            }
        }
    }
    match_calc_type!([CpuCalculator, AccurateCalculator, GpuCalculator]);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn AFC_AddSphereWaveSource(
    handle: *mut c_void,
    calc_type: i32,
    source: SphereWaveSource,
) {
    let calc_type = CalculatorType::from_i32(calc_type);
    macro_rules! match_calc_type {
        ([$($st:ident ),*]) => {
            match calc_type {
                $(CalculatorType::$st => {
                        let mut calc: Box<$st<SphereWaveSource>> = Box::from_raw(handle as *mut _);
                        (*calc).add_wave_source(source);
                        forget(calc);
                },)*
            }
        }
    }
    match_calc_type!([CpuCalculator, AccurateCalculator, GpuCalculator]);
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn AFC_AddT4010A1(handle: *mut c_void, calc_type: i32, source: T4010A1) {
    let calc_type = CalculatorType::from_i32(calc_type);
    macro_rules! match_calc_type {
        ([$($st:ident ),*]) => {
            match calc_type {
                $(CalculatorType::$st => {
                        let mut calc: Box<$st<T4010A1>> = Box::from_raw(handle as *mut _);
                        (*calc).add_wave_source(source);
                        forget(calc);
                },)*
            }
        }
    }
    match_calc_type!([CpuCalculator, AccurateCalculator, GpuCalculator]);
}

#[no_mangle]
pub unsafe extern "C" fn AFC_GetWaveSources(
    handle: *mut c_void,
    out: *mut *mut c_void,
    calc_type: i32,
    source_type: i32,
) -> u64 {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
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
    macro_rules! match_calc_type {
        ([$($ct:ident ),*]) => {
            match calc_type {
                $(CalculatorType::$ct => {
                    gen_match_src_type!([SphereWaveSource, T4010A1], $ct)
                },)*
            }
        }
    }
    match_calc_type!([CpuCalculator, AccurateCalculator, GpuCalculator])
}

#[no_mangle]
pub unsafe extern "C" fn AFC_Calculate(
    handle: *mut c_void,
    obs_handle: *mut c_void,
    field_handle: *mut c_void,
    out: *mut *const c_void,
    calc_type: i32,
    source_type: i32,
    obs_type: i32,
    field_type: i32,
) -> u64 {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    let obs_type = ObserveAreaType::from_i32(obs_type);
    let field_type = FieldType::from_i32(field_type);

    macro_rules! gen_match_field_type {
        ([$($field_type:ident),*], $calc: ident, $obs_type: ty) => {
            match field_type {
                $(FieldType::$field_type => {
                    let area: Box< $obs_type> = Box::from_raw(obs_handle as *mut _);
                    let mut field: Box<$field_type<f32>> = Box::from_raw(field_handle as *mut _);
                    $calc.calculate(&*area, &mut *field);
                    let res = (*field).buffer();
                    let len = res.len();
                    let ptr = res.as_ptr() as *const f32;
                    forget(area);
                    forget(field);
                    *out = ptr as *const c_void;
                    len as u64
                },)*
                _ => unreachable!()
            }
        }
    }
    macro_rules! gen_match_obs_type {
        ([$(($obs_type:ident, $obs_type_name:ty)),*], $calc: ident) => {
            match obs_type {
                $(ObserveAreaType::$obs_type => {
                    gen_match_field_type!([PressureField, PowerField], $calc, $obs_type_name)
                },)*
            }
        }
    }
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                    let len = gen_match_obs_type!(
                        [(GridArea1D, GridArea<N1>),
                        (GridArea2D, GridArea<N1>),
                        (GridArea3D, GridArea<N1>),
                        (Scatter, ScatterArea)],
                        calc);
                    forget(calc);
                    len
                },)*
            }
        }
    }
    macro_rules! match_calc_type {
        ([$($ct:ident ),*]) => {
            match calc_type {
                $(CalculatorType::$ct => {
                    gen_match_src_type!([SphereWaveSource, T4010A1], $ct)
                },)*
            }
        }
    }
    match_calc_type!([CpuCalculator, AccurateCalculator, GpuCalculator])
}

#[no_mangle]
pub unsafe extern "C" fn AFC_CalculateComplex(
    handle: *mut c_void,
    obs_handle: *mut c_void,
    field_handle: *mut c_void,
    out: *mut *const c_void,
    calc_type: i32,
    source_type: i32,
    obs_type: i32,
    field_type: i32,
) -> u64 {
    let calc_type = CalculatorType::from_i32(calc_type);
    let source_type = SourceType::from_i32(source_type);
    let obs_type = ObserveAreaType::from_i32(obs_type);
    let field_type = FieldType::from_i32(field_type);

    macro_rules! gen_match_field_type {
        ([$($field_type:ident),*], $calc: ident, $obs_type: ty) => {
            match field_type {
                $(FieldType::$field_type => {
                    let area: Box< $obs_type> = Box::from_raw(obs_handle as *mut _);
                    let mut field: Box<$field_type<Complex>> = Box::from_raw(field_handle as *mut _);
                    $calc.calculate(&*area, &mut *field);
                    let res = (*field).buffer();
                    let len = res.len();
                    let ptr = res.as_ptr() as *const Complex;
                    forget(area);
                    forget(field);
                    *out = ptr as *const c_void;
                    len as u64
                },)*
                _ => unreachable!()
            }
        }
    }
    macro_rules! gen_match_obs_type {
        ([$(($obs_type:ident, $obs_type_name:ty)),*], $calc: ident) => {
            match obs_type {
                $(ObserveAreaType::$obs_type => {
                    gen_match_field_type!([ComplexPressureField], $calc, $obs_type_name)
                },)*
            }
        }
    }
    macro_rules! gen_match_src_type {
        ([$( $src_type:ident),*], $calc_type: ident) => {
            match source_type {
                $(SourceType::$src_type => {
                    let mut calc: Box<$calc_type<$src_type>> = Box::from_raw(handle as *mut _);
                    let len = gen_match_obs_type!(
                        [(GridArea1D, GridArea<N1>),
                        (GridArea2D, GridArea<N1>),
                        (GridArea3D, GridArea<N1>),
                        (Scatter, ScatterArea)],
                        calc);
                    forget(calc);
                    len
                },)*
            }
        }
    }
    macro_rules! match_calc_type {
        ([$($ct:ident ),*]) => {
            match calc_type {
                $(CalculatorType::$ct => {
                    gen_match_src_type!([SphereWaveSource, T4010A1], $ct)
                },)*
            }
        }
    }
    match_calc_type!([CpuCalculator, AccurateCalculator, GpuCalculator])
}
