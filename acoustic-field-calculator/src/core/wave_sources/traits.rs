/*
 * File: traits.rs
 * Project: core
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{Float, Vector3};

macro_rules! impl_sync {
    ($name: ident, $( $def: item ),* ) => {
        #[cfg(feature = "parallel")]
        pub trait $name: Sync{
            $( $def )*
        }
        #[cfg(not(feature = "parallel"))]
        pub trait $name{
            $( $def )*
        }
    };
}

macro_rules! getset {
    ((getter=$name:ident), $type:ty ) => {
        fn $name(&self) -> $type;
    };
    ((setter=$name:ident), $type:ty ) => {
        fn $name(&mut self, value: $type);
    };
    ((getter=$name_getter:ident, setter=$name_setter:ident), $type:ty ) => {
        getset!((getter = $name_getter), $type);
        getset!((setter = $name_setter), $type);
    };
}

macro_rules! impl_getset {
    ((get=$getter_name:ident, field=$field_name:ident), $type:ty ) => {
        fn $getter_name(&self) -> $type {
            self.$field_name
        }
    };
    ((set=$setter_name:ident, field=$field_name:ident), $type:ty ) => {
        fn $setter_name(&mut self, value: $type) {
            self.$field_name = value;
        }
    };
    ((get=$getter_name:ident, set=$setter_name:ident, field=$field_name:ident), $type:ty ) => {
        impl_getset!((get = $getter_name, field = $field_name), $type);
        impl_getset!((set = $setter_name, field = $field_name), $type);
    };
}

impl_sync!(
    WaveSource,
    /// Get directivity
    ///
    /// # Arguments
    ///
    /// * `theta` - angle
    fn directivity(theta: Float) -> Float;,
    getset!((setter = frequency), Float);,
    // getset!((getter = wavenumber), Float);,
    getset!((getter = position, setter = set_position), Vector3);,
    getset!((getter = direction, setter = set_direction), Vector3);,
    getset!((getter = phase, setter = set_phase), Float);,
    getset!((getter = amp, setter = set_amp), Float);
);
