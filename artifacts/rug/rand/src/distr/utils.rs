// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Math helper functions

#[cfg(feature = "simd_support")]
use core::simd::prelude::*;
#[cfg(feature = "simd_support")]
use core::simd::{LaneCount, SimdElement, SupportedLaneCount};

pub(crate) trait WideningMultiply<RHS = Self> {
    type Output;

    fn wmul(self, x: RHS) -> Self::Output;
}

macro_rules! wmul_impl {
    ($ty:ty, $wide:ty, $shift:expr) => {
        impl WideningMultiply for $ty {
            type Output = ($ty, $ty);

            #[inline(always)]
            fn wmul(self, x: $ty) -> Self::Output {
                let tmp = (self as $wide) * (x as $wide);
                ((tmp >> $shift) as $ty, tmp as $ty)
            }
        }
    };

    // simd bulk implementation
    ($(($ty:ident, $wide:ty),)+, $shift:expr) => {
        $(
            impl WideningMultiply for $ty {
                type Output = ($ty, $ty);

                #[inline(always)]
                fn wmul(self, x: $ty) -> Self::Output {
                    // For supported vectors, this should compile to a couple
                    // supported multiply & swizzle instructions (no actual
                    // casting).
                    // TODO: optimize
                    let y: $wide = self.cast();
                    let x: $wide = x.cast();
                    let tmp = y * x;
                    let hi: $ty = (tmp >> Simd::splat($shift)).cast();
                    let lo: $ty = tmp.cast();
                    (hi, lo)
                }
            }
        )+
    };
}
wmul_impl! { u8, u16, 8 }
wmul_impl! { u16, u32, 16 }
wmul_impl! { u32, u64, 32 }
wmul_impl! { u64, u128, 64 }

// This code is a translation of the __mulddi3 function in LLVM's
// compiler-rt. It is an optimised variant of the common method
// `(a + b) * (c + d) = ac + ad + bc + bd`.
//
// For some reason LLVM can optimise the C version very well, but
// keeps shuffling registers in this Rust translation.
macro_rules! wmul_impl_large {
    ($ty:ty, $half:expr) => {
        impl WideningMultiply for $ty {
            type Output = ($ty, $ty);

            #[inline(always)]
            fn wmul(self, b: $ty) -> Self::Output {
                const LOWER_MASK: $ty = !0 >> $half;
                let mut low = (self & LOWER_MASK).wrapping_mul(b & LOWER_MASK);
                let mut t = low >> $half;
                low &= LOWER_MASK;
                t += (self >> $half).wrapping_mul(b & LOWER_MASK);
                low += (t & LOWER_MASK) << $half;
                let mut high = t >> $half;
                t = low >> $half;
                low &= LOWER_MASK;
                t += (b >> $half).wrapping_mul(self & LOWER_MASK);
                low += (t & LOWER_MASK) << $half;
                high += t >> $half;
                high += (self >> $half).wrapping_mul(b >> $half);

                (high, low)
            }
        }
    };

    // simd bulk implementation
    (($($ty:ty,)+) $scalar:ty, $half:expr) => {
        $(
            impl WideningMultiply for $ty {
                type Output = ($ty, $ty);

                #[inline(always)]
                fn wmul(self, b: $ty) -> Self::Output {
                    // needs wrapping multiplication
                    let lower_mask = <$ty>::splat(!0 >> $half);
                    let half = <$ty>::splat($half);
                    let mut low = (self & lower_mask) * (b & lower_mask);
                    let mut t = low >> half;
                    low &= lower_mask;
                    t += (self >> half) * (b & lower_mask);
                    low += (t & lower_mask) << half;
                    let mut high = t >> half;
                    t = low >> half;
                    low &= lower_mask;
                    t += (b >> half) * (self & lower_mask);
                    low += (t & lower_mask) << half;
                    high += t >> half;
                    high += (self >> half) * (b >> half);

                    (high, low)
                }
            }
        )+
    };
}
wmul_impl_large! { u128, 64 }

macro_rules! wmul_impl_usize {
    ($ty:ty) => {
        impl WideningMultiply for usize {
            type Output = (usize, usize);

            #[inline(always)]
            fn wmul(self, x: usize) -> Self::Output {
                let (high, low) = (self as $ty).wmul(x as $ty);
                (high as usize, low as usize)
            }
        }
    };
}
#[cfg(target_pointer_width = "16")]
wmul_impl_usize! { u16 }
#[cfg(target_pointer_width = "32")]
wmul_impl_usize! { u32 }
#[cfg(target_pointer_width = "64")]
wmul_impl_usize! { u64 }

#[cfg(feature = "simd_support")]
mod simd_wmul {
    use super::*;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    wmul_impl! {
        (u8x4, u16x4),
        (u8x8, u16x8),
        (u8x16, u16x16),
        (u8x32, u16x32),
        (u8x64, Simd<u16, 64>),,
        8
    }

    wmul_impl! { (u16x2, u32x2),, 16 }
    wmul_impl! { (u16x4, u32x4),, 16 }
    #[cfg(not(target_feature = "sse2"))]
    wmul_impl! { (u16x8, u32x8),, 16 }
    #[cfg(not(target_feature = "avx2"))]
    wmul_impl! { (u16x16, u32x16),, 16 }
    #[cfg(not(target_feature = "avx512bw"))]
    wmul_impl! { (u16x32, Simd<u32, 32>),, 16 }

    // 16-bit lane widths allow use of the x86 `mulhi` instructions, which
    // means `wmul` can be implemented with only two instructions.
    #[allow(unused_macros)]
    macro_rules! wmul_impl_16 {
        ($ty:ident, $mulhi:ident, $mullo:ident) => {
            impl WideningMultiply for $ty {
                type Output = ($ty, $ty);

                #[inline(always)]
                fn wmul(self, x: $ty) -> Self::Output {
                    let hi = unsafe { $mulhi(self.into(), x.into()) }.into();
                    let lo = unsafe { $mullo(self.into(), x.into()) }.into();
                    (hi, lo)
                }
            }
        };
    }

    #[cfg(target_feature = "sse2")]
    wmul_impl_16! { u16x8, _mm_mulhi_epu16, _mm_mullo_epi16 }
    #[cfg(target_feature = "avx2")]
    wmul_impl_16! { u16x16, _mm256_mulhi_epu16, _mm256_mullo_epi16 }
    #[cfg(target_feature = "avx512bw")]
    wmul_impl_16! { u16x32, _mm512_mulhi_epu16, _mm512_mullo_epi16 }

    wmul_impl! {
        (u32x2, u64x2),
        (u32x4, u64x4),
        (u32x8, u64x8),
        (u32x16, Simd<u64, 16>),,
        32
    }

    wmul_impl_large! { (u64x2, u64x4, u64x8,) u64, 32 }
}

/// Helper trait when dealing with scalar and SIMD floating point types.
pub(crate) trait FloatSIMDUtils {
    // `PartialOrd` for vectors compares lexicographically. We want to compare all
    // the individual SIMD lanes instead, and get the combined result over all
    // lanes. This is possible using something like `a.lt(b).all()`, but we
    // implement it as a trait so we can write the same code for `f32` and `f64`.
    // Only the comparison functions we need are implemented.
    fn all_lt(self, other: Self) -> bool;
    fn all_le(self, other: Self) -> bool;
    fn all_finite(self) -> bool;

    type Mask;
    fn gt_mask(self, other: Self) -> Self::Mask;

    // Decrease all lanes where the mask is `true` to the next lower value
    // representable by the floating-point type. At least one of the lanes
    // must be set.
    fn decrease_masked(self, mask: Self::Mask) -> Self;

    // Convert from int value. Conversion is done while retaining the numerical
    // value, not by retaining the binary representation.
    type UInt;
    fn cast_from_int(i: Self::UInt) -> Self;
}

#[cfg(test)]
pub(crate) trait FloatSIMDScalarUtils: FloatSIMDUtils {
    type Scalar;

    fn replace(self, index: usize, new_value: Self::Scalar) -> Self;
    fn extract_lane(self, index: usize) -> Self::Scalar;
}

/// Implement functions on f32/f64 to give them APIs similar to SIMD types
pub(crate) trait FloatAsSIMD: Sized {
    #[cfg(test)]
    const LEN: usize = 1;

    #[inline(always)]
    fn splat(scalar: Self) -> Self {
        scalar
    }
}

pub(crate) trait IntAsSIMD: Sized {
    #[inline(always)]
    fn splat(scalar: Self) -> Self {
        scalar
    }
}

impl IntAsSIMD for u32 {}
impl IntAsSIMD for u64 {}

pub(crate) trait BoolAsSIMD: Sized {
    fn any(self) -> bool;
}

impl BoolAsSIMD for bool {
    #[inline(always)]
    fn any(self) -> bool {
        self
    }
}

macro_rules! scalar_float_impl {
    ($ty:ident, $uty:ident) => {
        impl FloatSIMDUtils for $ty {
            type Mask = bool;
            type UInt = $uty;

            #[inline(always)]
            fn all_lt(self, other: Self) -> bool {
                self < other
            }

            #[inline(always)]
            fn all_le(self, other: Self) -> bool {
                self <= other
            }

            #[inline(always)]
            fn all_finite(self) -> bool {
                self.is_finite()
            }

            #[inline(always)]
            fn gt_mask(self, other: Self) -> Self::Mask {
                self > other
            }

            #[inline(always)]
            fn decrease_masked(self, mask: Self::Mask) -> Self {
                debug_assert!(mask, "At least one lane must be set");
                <$ty>::from_bits(self.to_bits() - 1)
            }

            #[inline]
            fn cast_from_int(i: Self::UInt) -> Self {
                i as $ty
            }
        }

        #[cfg(test)]
        impl FloatSIMDScalarUtils for $ty {
            type Scalar = $ty;

            #[inline]
            fn replace(self, index: usize, new_value: Self::Scalar) -> Self {
                debug_assert_eq!(index, 0);
                new_value
            }

            #[inline]
            fn extract_lane(self, index: usize) -> Self::Scalar {
                debug_assert_eq!(index, 0);
                self
            }
        }

        impl FloatAsSIMD for $ty {}
    };
}

scalar_float_impl!(f32, u32);
scalar_float_impl!(f64, u64);

#[cfg(feature = "simd_support")]
macro_rules! simd_impl {
    ($fty:ident, $uty:ident) => {
        impl<const LANES: usize> FloatSIMDUtils for Simd<$fty, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Mask = Mask<<$fty as SimdElement>::Mask, LANES>;
            type UInt = Simd<$uty, LANES>;

            #[inline(always)]
            fn all_lt(self, other: Self) -> bool {
                self.simd_lt(other).all()
            }

            #[inline(always)]
            fn all_le(self, other: Self) -> bool {
                self.simd_le(other).all()
            }

            #[inline(always)]
            fn all_finite(self) -> bool {
                self.is_finite().all()
            }

            #[inline(always)]
            fn gt_mask(self, other: Self) -> Self::Mask {
                self.simd_gt(other)
            }

            #[inline(always)]
            fn decrease_masked(self, mask: Self::Mask) -> Self {
                // Casting a mask into ints will produce all bits set for
                // true, and 0 for false. Adding that to the binary
                // representation of a float means subtracting one from
                // the binary representation, resulting in the next lower
                // value representable by $fty. This works even when the
                // current value is infinity.
                debug_assert!(mask.any(), "At least one lane must be set");
                Self::from_bits(self.to_bits() + mask.to_int().cast())
            }

            #[inline]
            fn cast_from_int(i: Self::UInt) -> Self {
                i.cast()
            }
        }

        #[cfg(test)]
        impl<const LANES: usize> FloatSIMDScalarUtils for Simd<$fty, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Scalar = $fty;

            #[inline]
            fn replace(mut self, index: usize, new_value: Self::Scalar) -> Self {
                self.as_mut_array()[index] = new_value;
                self
            }

            #[inline]
            fn extract_lane(self, index: usize) -> Self::Scalar {
                self.as_array()[index]
            }
        }
    };
}

#[cfg(feature = "simd_support")]
simd_impl!(f32, u32);
#[cfg(feature = "simd_support")]
simd_impl!(f64, u64);

#[cfg(test)]
mod tests_llm_16_239 {
    use super::*;

use crate::*;

    #[test]
    fn test_all_finite() {
        assert!(f32::INFINITY.all_finite() == false);
        assert!(f32::NEG_INFINITY.all_finite() == false);
        assert!(f32::NAN.all_finite() == false);
        assert!(1.0f32.all_finite() == true);
        assert!((-1.0f32).all_finite() == true);
        assert!((0.0f32).all_finite() == true);
    }
}

#[cfg(test)]
mod tests_llm_16_240 {
    use super::*;

use crate::*;
    use crate::distr::utils::FloatSIMDUtils;

    #[test]
    fn test_all_le() {
        let a: f32 = 1.0;
        let b: f32 = 2.0;
        let c: f32 = 1.0;

        // Test case where all elements are less than or equal to
        assert!(a.all_le(b));

        // Test case where all elements are equal
        assert!(a.all_le(c));

        // Test case where a is greater than b
        assert!(!b.all_le(a));
    }
}

#[cfg(test)]
mod tests_llm_16_241 {
    use super::*;

use crate::*;

    #[test]
    fn test_all_lt() {
        let a: f32 = 1.0;
        let b: f32 = 2.0;
        let c: f32 = 1.0;

        assert!(a.all_lt(b));
        assert!(!b.all_lt(a));
        assert!(!a.all_lt(c));
    }
}

#[cfg(test)]
mod tests_llm_16_242 {
    use super::*;

use crate::*;
    use crate::distr::utils::FloatSIMDUtils;

    #[test]
    fn test_cast_from_int() {
        let int_value: u32 = 42;
        let float_value: f32 = f32::cast_from_int(int_value);
        assert_eq!(float_value, 42.0);
    }

    #[test]
    fn test_cast_from_zero() {
        let int_value: u32 = 0;
        let float_value: f32 = f32::cast_from_int(int_value);
        assert_eq!(float_value, 0.0);
    }

    #[test]
    fn test_cast_from_large_int() {
        let int_value: u32 = 1_000_000;
        let float_value: f32 = f32::cast_from_int(int_value);
        assert_eq!(float_value, 1_000_000.0);
    }

    #[test]
    fn test_cast_from_negative_int() {
        let int_value: u32 = u32::MAX; // Maximum unsigned int
        let float_value: f32 = f32::cast_from_int(int_value);
        assert_eq!(float_value, u32::MAX as f32);
    }
}

#[cfg(test)]
mod tests_llm_16_244 {
    use super::*;

use crate::*;
    use crate::distr::utils::FloatSIMDUtils;

    #[test]
    fn test_gt_mask() {
        let a: f32 = 3.5;
        let b: f32 = 2.5;
        let c: f32 = 3.5;

        assert_eq!(a.gt_mask(b), true);
        assert_eq!(a.gt_mask(c), false);
        assert_eq!(b.gt_mask(a), false);
    }
}

#[cfg(test)]
mod tests_llm_16_246 {
    use super::*;

use crate::*;
    use distr::utils::FloatSIMDUtils;

    #[test]
    fn test_all_finite() {
        let finite_value: f64 = 1.0;
        let infinite_value: f64 = f64::INFINITY;
        let nan_value: f64 = f64::NAN;

        assert!(finite_value.all_finite());
        assert!(!infinite_value.all_finite());
        assert!(!nan_value.all_finite());
    }
}

#[cfg(test)]
mod tests_llm_16_248 {
    use super::*;

use crate::*;

    #[test]
    fn test_all_lt() {
        let a: f64 = 3.0;
        let b: f64 = 5.0;
        let c: f64 = 3.0;

        assert!(a.all_lt(b)); // 3.0 < 5.0 should return true
        assert!(!a.all_lt(c)); // 3.0 < 3.0 should return false
        assert!(b.all_lt(f64::INFINITY)); // 5.0 < infinity should return true
        assert!(!b.all_lt(f64::NAN)); // 5.0 < NaN should return false
    }
}

#[cfg(test)]
mod tests_llm_16_251 {
    use super::*;

use crate::*;
    use crate::distr::utils::FloatSIMDUtils; // Adjust the import path as necessary

    #[test]
    fn test_gt_mask() {
        let a: f64 = 3.0;
        let b: f64 = 2.0;
        let result: <f64 as FloatSIMDUtils>::Mask = a.gt_mask(b);
        assert_eq!(result, true); // Assuming the Mask is a boolean
    }

    #[test]
    fn test_gt_mask_equal() {
        let a: f64 = 2.0;
        let b: f64 = 2.0;
        let result: <f64 as FloatSIMDUtils>::Mask = a.gt_mask(b);
        assert_eq!(result, false); // Assuming the Mask is a boolean
    }

    #[test]
    fn test_gt_mask_less() {
        let a: f64 = 1.0;
        let b: f64 = 2.0;
        let result: <f64 as FloatSIMDUtils>::Mask = a.gt_mask(b);
        assert_eq!(result, false); // Assuming the Mask is a boolean
    }
}

#[cfg(test)]
mod tests_llm_16_303 {
    use super::*;

use crate::*;
    use crate::distr::utils::WideningMultiply; // Adjust the import path as necessary for the current crate structure

    #[test]
    fn test_wmul() {
        let a: u16 = 5;
        let b: u16 = 10;
        let (high, low) = a.wmul(b);
        assert_eq!(high, 0);
        assert_eq!(low, 50);
    }

    #[test]
    fn test_wmul_large_values() {
        let a: u16 = u16::MAX;
        let b: u16 = u16::MAX;
        let (high, low) = a.wmul(b);
        assert_eq!(high, 65535);
        assert_eq!(low, 1);
    }

    #[test]
    fn test_wmul_edge_case_zero() {
        let a: u16 = 0;
        let b: u16 = 10;
        let (high, low) = a.wmul(b);
        assert_eq!(high, 0);
        assert_eq!(low, 0);
    }

    #[test]
    fn test_wmul_edge_case_identity() {
        let a: u16 = 1;
        let b: u16 = 10;
        let (high, low) = a.wmul(b);
        assert_eq!(high, 0);
        assert_eq!(low, 10);
    }
}

#[cfg(test)]
mod tests_llm_16_306 {
    use super::*; // Adjust the import as necessary to access wmul

use crate::*;
    use crate::distr::utils::WideningMultiply;

    #[test]
    fn test_wmul() {
        // Test case 1
        let a: u32 = 3;
        let b: u32 = 4;
        let (high, low) = a.wmul(b);
        assert_eq!(high, 0);
        assert_eq!(low, 12);

        // Test case 2
        let a: u32 = 0xFFFF_FFFF; // Max u32 value
        let b: u32 = 2;
        let (high, low) = a.wmul(b);
        assert_eq!(high, 0xFFFF_FFFF);
        assert_eq!(low, 0xFFFFFFFE);

        // Test case 3
        let a: u32 = 1;
        let b: u32 = 1;
        let (high, low) = a.wmul(b);
        assert_eq!(high, 0);
        assert_eq!(low, 1);
    }
}

#[cfg(test)]
mod tests_llm_16_312 {
    use super::*;

use crate::*;
    use crate::distr::utils::WideningMultiply;

    #[test]
    fn test_wmul() {
        let a: u64 = 10;
        let b: u64 = 20;
        let (high, low) = a.wmul(b);
        assert_eq!(high, 0);
        assert_eq!(low, 200);
    }

    #[test]
    fn test_wmul_with_large_numbers() {
        let a: u64 = u64::MAX;
        let b: u64 = 2;
        let (high, low) = a.wmul(b);
        assert_eq!(high, 1);
        assert_eq!(low, u64::MAX - 1);
    }

    #[test]
    fn test_wmul_with_zero() {
        let a: u64 = 0;
        let b: u64 = 10;
        let (high, low) = a.wmul(b);
        assert_eq!(high, 0);
        assert_eq!(low, 0);
    }

    #[test]
    fn test_wmul_with_one() {
        let a: u64 = 1;
        let b: u64 = 1;
        let (high, low) = a.wmul(b);
        assert_eq!(high, 0);
        assert_eq!(low, 1);
    }
}

#[cfg(test)]
mod tests_llm_16_319 {
    use super::*;

use crate::*;
    use crate::distr::utils::WideningMultiply;

    #[test]
    fn test_wmul() {
        let a: usize = 10;
        let b: usize = 20;
        let (high, low) = a.wmul(b);
        assert_eq!(high, 0);  // Change this according to expected high value
        assert_eq!(low, 200);  // Change this according to expected low value
    }

    #[test]
    fn test_wmul_zero() {
        let a: usize = 0;
        let b: usize = 20;
        let (high, low) = a.wmul(b);
        assert_eq!(high, 0);
        assert_eq!(low, 0);
    }

    #[test]
    fn test_wmul_large_values() {
        let a: usize = usize::MAX;
        let b: usize = 2;
        let (high, low) = a.wmul(b);
        assert_eq!(high, usize::MAX); // Change according to expected high value
        assert_eq!(low, usize::MAX - 1); // Change according to expected low value
    }
}

#[cfg(test)]
mod tests_llm_16_379 {
    use super::*;

use crate::*;
    use crate::distr::utils::FloatAsSIMD;

    #[test]
    fn test_splat() {
        let scalar = 5.0; // Example scalar value
        let result = FloatAsSIMD::splat(scalar);
        assert_eq!(result, scalar);
    }
}
