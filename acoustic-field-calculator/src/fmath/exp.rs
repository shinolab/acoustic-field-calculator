/*
 * File: exp.rs
 * Project: fmath
 * Created Date: 22/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::core::Float;

#[cfg(all(feature = "fmath", not(feature = "accurate")))]
mod fmath {
    use crate::core::Float;
    use once_cell::sync::Lazy;

    const EXP_TABLE_SIZE_BIT: usize = 10;
    const EXP_TABLE_SIZE: usize = 1 << EXP_TABLE_SIZE_BIT;

    #[cfg(not(feature = "double"))]
    mod def {
        use crate::core::Float;
        pub type UINT = u32;
        pub type INT = i32;
        pub const FRACTION_BIT_SIZE: INT = 23;
        pub const MAX_D: Float = 88.722839;
        pub const MIN_D: Float = -103.972081;
        pub const BIAS: INT = 127;
    }
    #[cfg(feature = "double")]
    mod def {
        use crate::core::Float;
        pub type UINT = u64;
        pub type INT = i64;
        pub const FRACTION_BIT_SIZE: INT = 52;
        pub const MAX_D: Float = 709.78271289338397;
        pub const MIN_D: Float = -708.39641853226408;
        pub const BIAS: INT = 1023;
    }

    use def::*;

    const fn mask(x: INT) -> UINT {
        ((1 as UINT) << x) - (1 as UINT)
    }

    #[repr(C)]
    union float {
        f: Float,
        u: UINT,
    }

    static EXP_FRAC_TABLE: Lazy<[UINT; EXP_TABLE_SIZE]> = Lazy::new(|| {
        let mut exp_frac: [UINT; EXP_TABLE_SIZE] = [0; EXP_TABLE_SIZE];
        for (i, r) in exp_frac.iter_mut().enumerate() {
            let y = float {
                f: (2.0 as Float).powf(i as Float / EXP_TABLE_SIZE as Float),
            };
            let bit = unsafe { y.u };
            *r = bit & mask(FRACTION_BIT_SIZE);
        }
        exp_frac
    });

    const LN2: Float = 0.69314718056;
    const ALPHA: Float = (1 << EXP_TABLE_SIZE_BIT) as Float / LN2;
    const RECIPROCAL_ALPHA: Float = LN2 / (1 << EXP_TABLE_SIZE_BIT) as Float;

    #[inline(always)]
    pub fn exp(x: Float) -> Float {
        if x <= MIN_D {
            return 0.;
        }

        if x >= MAX_D {
            return Float::INFINITY;
        }

        let xp = (x * ALPHA) as INT;
        let f = x - xp as Float * RECIPROCAL_ALPHA;

        let u = xp >> EXP_TABLE_SIZE_BIT;
        let v = xp & mask(EXP_TABLE_SIZE_BIT as INT) as INT;
        let i = ((u + BIAS) << FRACTION_BIT_SIZE) as UINT | EXP_FRAC_TABLE[v as usize];
        let e = float { u: i };
        #[cfg(feature = "double")]
        {
            unsafe { (1.0 + f * (1.0 + f / 2.0)) * e.f }
        }
        #[cfg(not(feature = "double"))]
        {
            unsafe { (1.0 + f) * e.f }
        }
    }
}

#[inline(always)]
#[allow(clippy::many_single_char_names)]
pub fn exp(x: Float) -> Float {
    #[cfg(all(feature = "fmath", not(feature = "accurate")))]
    {
        fmath::exp(x)
    }
    #[cfg(not(all(feature = "fmath", not(feature = "accurate"))))]
    {
        x.exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    const MIN: Float = -104.0;
    const MAX: Float = 89.0;
    const STEP: Float = 0.01;
    const SIZE: usize = ((MAX - MIN) / STEP) as usize;

    #[test]
    fn exp_test() {
        let _ = (0..SIZE)
            .map(|n| MIN + n as Float * STEP)
            .map(|x| approx_eq!(Float, x.exp(), exp(x)))
            .collect::<Vec<_>>();
    }
}
