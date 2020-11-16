/*
 * File: trigonometric.rs
 * Project: fmath
 * Created Date: 22/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::core::Float;

#[cfg(all(feature = "fmath", not(feature = "accurate")))]
mod fmath {
    use crate::core::Float;

    #[cfg(not(feature = "double"))]
    use std::f32::consts::PI;
    #[cfg(feature = "double")]
    use std::f64::consts::PI;

    #[inline(always)]
    pub fn acos(x: Float) -> Float {
        let neg = if x.is_sign_negative() { 1.0 } else { 0.0 };
        let x = x.abs();
        let ret = -0.0187293;
        let ret = ret * x;
        let ret = ret + 0.0742610;
        let ret = ret * x;
        let ret = ret - 0.2121144;
        let ret = ret * x;
        let ret = ret + 1.5707288;
        let ret = ret * (1.0 - x).sqrt();
        let ret = ret - 2.0 * neg * ret;
        neg * PI + ret
    }
}

#[inline(always)]
pub fn acos(x: Float) -> Float {
    #[cfg(all(feature = "fmath", not(feature = "accurate")))]
    {
        fmath::acos(x)
    }
    #[cfg(not(all(feature = "fmath", not(feature = "accurate"))))]
    {
        x.acos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    const MIN: Float = -1.0;
    const MAX: Float = 1.0;
    const STEP: Float = 0.01;
    const SIZE: usize = ((MAX - MIN) / STEP) as usize;

    #[test]
    fn acos_test() {
        let _ = (0..SIZE)
            .map(|n| MIN + n as Float * STEP)
            .map(|x| approx_eq!(Float, x.acos(), acos(x)))
            .collect::<Vec<_>>();
    }
}
