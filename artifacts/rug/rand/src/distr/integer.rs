// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The implementations of the `StandardUniform` distribution for integer types.

use crate::distr::{Distribution, StandardUniform};
use crate::Rng;
#[cfg(all(target_arch = "x86", feature = "simd_support"))]
use core::arch::x86::__m512i;
#[cfg(target_arch = "x86")]
use core::arch::x86::{__m128i, __m256i};
#[cfg(all(target_arch = "x86_64", feature = "simd_support"))]
use core::arch::x86_64::__m512i;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{__m128i, __m256i};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8,
};
#[cfg(feature = "simd_support")]
use core::simd::*;

impl Distribution<u8> for StandardUniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        rng.next_u32() as u8
    }
}

impl Distribution<u16> for StandardUniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u16 {
        rng.next_u32() as u16
    }
}

impl Distribution<u32> for StandardUniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u32 {
        rng.next_u32()
    }
}

impl Distribution<u64> for StandardUniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u64 {
        rng.next_u64()
    }
}

impl Distribution<u128> for StandardUniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u128 {
        // Use LE; we explicitly generate one value before the next.
        let x = u128::from(rng.next_u64());
        let y = u128::from(rng.next_u64());
        (y << 64) | x
    }
}

macro_rules! impl_int_from_uint {
    ($ty:ty, $uty:ty) => {
        impl Distribution<$ty> for StandardUniform {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                rng.random::<$uty>() as $ty
            }
        }
    };
}

impl_int_from_uint! { i8, u8 }
impl_int_from_uint! { i16, u16 }
impl_int_from_uint! { i32, u32 }
impl_int_from_uint! { i64, u64 }
impl_int_from_uint! { i128, u128 }

macro_rules! impl_nzint {
    ($ty:ty, $new:path) => {
        impl Distribution<$ty> for StandardUniform {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                loop {
                    if let Some(nz) = $new(rng.random()) {
                        break nz;
                    }
                }
            }
        }
    };
}

impl_nzint!(NonZeroU8, NonZeroU8::new);
impl_nzint!(NonZeroU16, NonZeroU16::new);
impl_nzint!(NonZeroU32, NonZeroU32::new);
impl_nzint!(NonZeroU64, NonZeroU64::new);
impl_nzint!(NonZeroU128, NonZeroU128::new);

impl_nzint!(NonZeroI8, NonZeroI8::new);
impl_nzint!(NonZeroI16, NonZeroI16::new);
impl_nzint!(NonZeroI32, NonZeroI32::new);
impl_nzint!(NonZeroI64, NonZeroI64::new);
impl_nzint!(NonZeroI128, NonZeroI128::new);

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl Distribution<__m128i> for StandardUniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> __m128i {
        // NOTE: It's tempting to use the u128 impl here, but confusingly this
        // results in different code (return via rdx, r10 instead of rax, rdx
        // with u128 impl) and is much slower (+130 time). This version calls
        // impls::fill_bytes_via_next but performs well.

        let mut buf = [0_u8; core::mem::size_of::<__m128i>()];
        rng.fill_bytes(&mut buf);
        // x86 is little endian so no need for conversion

        // SAFETY: All byte sequences of `buf` represent values of the output type.
        unsafe { core::mem::transmute(buf) }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl Distribution<__m256i> for StandardUniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> __m256i {
        let mut buf = [0_u8; core::mem::size_of::<__m256i>()];
        rng.fill_bytes(&mut buf);
        // x86 is little endian so no need for conversion

        // SAFETY: All byte sequences of `buf` represent values of the output type.
        unsafe { core::mem::transmute(buf) }
    }
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd_support"
))]
impl Distribution<__m512i> for StandardUniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> __m512i {
        let mut buf = [0_u8; core::mem::size_of::<__m512i>()];
        rng.fill_bytes(&mut buf);
        // x86 is little endian so no need for conversion

        // SAFETY: All byte sequences of `buf` represent values of the output type.
        unsafe { core::mem::transmute(buf) }
    }
}

#[cfg(feature = "simd_support")]
macro_rules! simd_impl {
    ($($ty:ty),+) => {$(
        /// Requires nightly Rust and the [`simd_support`] feature
        ///
        /// [`simd_support`]: https://github.com/rust-random/rand#crate-features
        #[cfg(feature = "simd_support")]
        impl<const LANES: usize> Distribution<Simd<$ty, LANES>> for StandardUniform
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Simd<$ty, LANES> {
                let mut vec = Simd::default();
                rng.fill(vec.as_mut_array().as_mut_slice());
                vec
            }
        }
    )+};
}

#[cfg(feature = "simd_support")]
simd_impl!(u8, i8, u16, i16, u32, i32, u64, i64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integers() {
        let mut rng = crate::test::rng(806);

        rng.sample::<i8, _>(StandardUniform);
        rng.sample::<i16, _>(StandardUniform);
        rng.sample::<i32, _>(StandardUniform);
        rng.sample::<i64, _>(StandardUniform);
        rng.sample::<i128, _>(StandardUniform);

        rng.sample::<u8, _>(StandardUniform);
        rng.sample::<u16, _>(StandardUniform);
        rng.sample::<u32, _>(StandardUniform);
        rng.sample::<u64, _>(StandardUniform);
        rng.sample::<u128, _>(StandardUniform);
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[test]
    fn x86_integers() {
        let mut rng = crate::test::rng(807);

        rng.sample::<__m128i, _>(StandardUniform);
        rng.sample::<__m256i, _>(StandardUniform);
        #[cfg(feature = "simd_support")]
        rng.sample::<__m512i, _>(StandardUniform);
    }

    #[test]
    fn value_stability() {
        fn test_samples<T: Copy + core::fmt::Debug + PartialEq>(zero: T, expected: &[T])
        where
            StandardUniform: Distribution<T>,
        {
            let mut rng = crate::test::rng(807);
            let mut buf = [zero; 3];
            for x in &mut buf {
                *x = rng.sample(StandardUniform);
            }
            assert_eq!(&buf, expected);
        }

        test_samples(0u8, &[9, 247, 111]);
        test_samples(0u16, &[32265, 42999, 38255]);
        test_samples(0u32, &[2220326409, 2575017975, 2018088303]);
        test_samples(
            0u64,
            &[
                11059617991457472009,
                16096616328739788143,
                1487364411147516184,
            ],
        );
        test_samples(
            0u128,
            &[
                296930161868957086625409848350820761097,
                145644820879247630242265036535529306392,
                111087889832015897993126088499035356354,
            ],
        );

        test_samples(0i8, &[9, -9, 111]);
        // Skip further i* types: they are simple reinterpretation of u* samples

        #[cfg(feature = "simd_support")]
        {
            // We only test a sub-set of types here and make assumptions about the rest.

            test_samples(
                u8x4::default(),
                &[
                    u8x4::from([9, 126, 87, 132]),
                    u8x4::from([247, 167, 123, 153]),
                    u8x4::from([111, 149, 73, 120]),
                ],
            );
            test_samples(
                u8x8::default(),
                &[
                    u8x8::from([9, 126, 87, 132, 247, 167, 123, 153]),
                    u8x8::from([111, 149, 73, 120, 68, 171, 98, 223]),
                    u8x8::from([24, 121, 1, 50, 13, 46, 164, 20]),
                ],
            );

            test_samples(
                i64x8::default(),
                &[
                    i64x8::from([
                        -7387126082252079607,
                        -2350127744969763473,
                        1487364411147516184,
                        7895421560427121838,
                        602190064936008898,
                        6022086574635100741,
                        -5080089175222015595,
                        -4066367846667249123,
                    ]),
                    i64x8::from([
                        9180885022207963908,
                        3095981199532211089,
                        6586075293021332726,
                        419343203796414657,
                        3186951873057035255,
                        5287129228749947252,
                        444726432079249540,
                        -1587028029513790706,
                    ]),
                    i64x8::from([
                        6075236523189346388,
                        1351763722368165432,
                        -6192309979959753740,
                        -7697775502176768592,
                        -4482022114172078123,
                        7522501477800909500,
                        -1837258847956201231,
                        -586926753024886735,
                    ]),
                ],
            );
        }
    }
}

#[cfg(test)]
mod tests_llm_16_333 {
    use crate::prelude::*;
    use crate::distr::StandardUniform;
    use core::num::NonZero;

    #[test]
    fn test_sample_non_zero_i32() {
        let mut rng = crate::thread_rng();
        let standard_uniform = StandardUniform;

        // Sample a non-zero i32 value
        let value: NonZero<i32> = standard_uniform.sample(&mut rng);

        // Ensure the value is non-zero
        assert!(value.get() != 0);
    }

    #[test]
    fn test_sample_multiple_non_zero_i32() {
        let mut rng = crate::thread_rng();
        let standard_uniform = StandardUniform;

        for _ in 0..100 {
            let value: NonZero<i32> = standard_uniform.sample(&mut rng);
            // Ensure each sampled value is non-zero
            assert!(value.get() != 0);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_335 {
    use crate::prelude::*;
    use crate::distr::StandardUniform;
    use core::num::NonZero;

    #[test]
    fn test_sample_non_zero_i8() {
        let mut rng = crate::thread_rng();
        let distr = StandardUniform;

        for _ in 0..1000 {
            let result: NonZero<i8> = distr.sample(&mut rng);
            assert!(result.get() != 0, "Sampled a zero value: {}", result);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_338 {
    use super::*;

use crate::*;
    use crate::rngs::ThreadRng;
    use crate::Rng;

    #[test]
    fn test_standard_uniform_sample_non_zero_u32() {
        let mut rng: ThreadRng = crate::thread_rng();
        let distribution = StandardUniform;

        for _ in 0..100 {
            let sample: NonZeroU32 = distribution.sample(&mut rng);
            assert!(sample.get() > 0);
        }
    }

    #[test]
    fn test_standard_uniform_sample_non_zero_u32_range() {
        let mut rng: ThreadRng = crate::thread_rng();
        let distribution = StandardUniform;

        for _ in 0..100 {
            let sample: NonZeroU32 = distribution.sample(&mut rng);
            assert!(sample.get() <= 0xFFFFFFFF); // NonZeroU32 can range from 1 to u32::MAX
        }
    }
}

#[cfg(test)]
mod tests_llm_16_340 {
    use super::*;

use crate::*;
    use crate::prelude::*;
    use std::num::NonZeroU8;

    #[test]
    fn test_sample_non_zero_u8() {
        let mut rng = crate::thread_rng();
        let distribution = StandardUniform;

        for _ in 0..100 {
            let result: NonZeroU8 = distribution.sample(&mut rng);
            assert!(result.get() != 0, "Sampled value should not be zero.");
        }
    }
}

#[cfg(test)]
mod tests_llm_16_342 {
    use super::*;

use crate::*;
    use crate::Rng; // Import Rng to use the random function
    use crate::rngs::StdRng; // Import a RNG type

    #[test]
    fn test_standard_uniform_sample() {
        let mut rng = StdRng::seed_from_u64(42); // Create a seeded random number generator
        let distribution = StandardUniform; // Instantiate the StandardUniform distribution

        // Test sampling for i16
        for _ in 0..100 {
            let value: i16 = distribution.sample(&mut rng); // Use the sample method
            assert!(value >= i16::MIN && value <= i16::MAX, "Value out of bounds: {}", value);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_344 {
    use super::*;

use crate::*;
    use crate::Rng;
    use crate::distr::StandardUniform;
    use crate::rngs::ThreadRng;

    #[test]
    fn test_sample_i64() {
        let mut rng: ThreadRng = crate::thread_rng();
        let standard_uniform = StandardUniform;

        // Sample a value
        let sampled_value: i64 = standard_uniform.sample(&mut rng);
        
        // Check that the sampled value is within the i64 range
        assert!(sampled_value >= i64::MIN && sampled_value <= i64::MAX);
    }
}

#[cfg(test)]
mod tests_llm_16_345 {
    use crate::prelude::*;
    use crate::distr::StandardUniform;
    use crate::Rng;

    #[test]
    fn test_sample_i8() {
        let mut rng = crate::thread_rng();
        let std_uniform = StandardUniform;

        for _ in 0..1000 {
            let sampled_value: i8 = std_uniform.sample(&mut rng);
            assert!(sampled_value >= i8::MIN && sampled_value <= i8::MAX);
        }
    }

    #[test]
    fn test_sample_i8_range() {
        let mut rng = crate::thread_rng();
        let std_uniform = StandardUniform;
        let sampled_value: i8 = std_uniform.sample(&mut rng);
        assert!(sampled_value >= i8::MIN && sampled_value <= i8::MAX);
    }
}
