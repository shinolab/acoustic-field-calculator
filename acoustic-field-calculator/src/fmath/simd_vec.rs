/*
 * File: simd_vec.rs
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

use crate::Vector3;

#[cfg(all(feature = "fmath", target_arch = "x86_64", not(feature = "accurate")))]
mod fmath {
    use super::Vector3;
    use std::arch::x86_64;

    #[cfg(not(feature = "double"))]
    mod def {
        use super::x86_64::__m128;
        use super::Vector3;

        pub union __Vector3 {
            pub simd_reg: __m128,
            pub vec: Vector3,
        }
        #[inline(always)]
        pub unsafe fn sub_simd(a: __m128, b: __m128) -> __m128 {
            super::x86_64::_mm_sub_ps(a, b)
        }
    }
    #[cfg(feature = "double")]
    mod def {
        use super::x86_64::__m256d;
        use super::Vector3;

        pub union __Vector3 {
            pub simd_reg: __m256d,
            pub vec: Vector3,
        }
        #[inline(always)]
        pub unsafe fn sub_simd(a: __m256d, b: __m256d) -> __m256d {
            super::x86_64::_mm256_sub_pd(a, b)
        }
    }

    use def::*;
    #[inline(always)]
    pub fn sub(a: Vector3, b: Vector3) -> Vector3 {
        unsafe {
            let _a = __Vector3 { vec: a };
            let _b = __Vector3 { vec: b };
            let r = __Vector3 {
                simd_reg: sub_simd(_a.simd_reg, _b.simd_reg),
            };
            r.vec
        }
    }
}

#[inline(always)]
pub fn sub(a: Vector3, b: Vector3) -> Vector3 {
    #[cfg(all(feature = "fmath", target_arch = "x86_64", not(feature = "accurate")))]
    {
        fmath::sub(a, b)
    }
    #[cfg(not(all(feature = "fmath", target_arch = "x86_64", not(feature = "accurate"))))]
    {
        a - b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_test() {
        let a = Vector3::new(10., 20., 30.);
        let b = Vector3::new(5., 6., 7.);
        assert_eq!(sub(a, b), a - b);
    }
}
