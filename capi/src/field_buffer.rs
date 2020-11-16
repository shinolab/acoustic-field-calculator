/*
 * File: field_buffer.rs
 * Project: src
 * Created Date: 21/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/10/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use acoustic_field_calculator::prelude::*;

use std::ffi::c_void;
use std::mem::forget;

macro_rules! impl_create_free {
    ($type_name: ty, $create_fn_name: ident, $free_fn_name: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $create_fn_name(out: *mut *mut c_void) {
            let buf = <$type_name>::new();
            let mut buf = Box::new(buf);
            let ptr = buf.as_mut() as *mut _;
            forget(buf);
            *out = ptr as *mut _;
        }

        #[no_mangle]
        pub unsafe extern "C" fn $free_fn_name(handle: *mut c_void) {
            let _buf: Box<$type_name> = Box::from_raw(handle as *mut _);
        }
    };
}

impl_create_free!(
    PressureField<f32>,
    AFC_CreatePressureField,
    AFC_FreePressureField
);

impl_create_free!(PowerField<f32>, AFC_CreatePowerField, AFC_FreePowerField);

impl_create_free!(
    ComplexPressureField<Complex>,
    AFC_CreateComplexPressureField,
    AFC_FreeComplexPressureField
);
