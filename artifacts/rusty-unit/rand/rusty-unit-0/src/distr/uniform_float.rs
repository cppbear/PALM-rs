// Copyright 2018-2020 Developers of the Rand project.
// Copyright 2017 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! `UniformFloat` implementation

use super::{Error, SampleBorrow, SampleUniform, UniformSampler};
use crate::distr::float::IntoFloat;
use crate::distr::utils::{BoolAsSIMD, FloatAsSIMD, FloatSIMDUtils, IntAsSIMD};
use crate::Rng;

#[cfg(feature = "simd_support")]
use core::simd::prelude::*;
// #[cfg(feature = "simd_support")]
// use core::simd::{LaneCount, SupportedLaneCount};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The back-end implementing [`UniformSampler`] for floating-point types.
///
/// Unless you are implementing [`UniformSampler`] for your own type, this type
/// should not be used directly, use [`Uniform`] instead.
///
/// # Implementation notes
///
/// `UniformFloat` implementations convert RNG output to a float in the range
/// `[1, 2)` via transmutation, map this to `[0, 1)`, then scale and translate
/// to the desired range. Values produced this way have what equals 23 bits of
/// random digits for an `f32` and 52 for an `f64`.
///
/// # Bias and range errors
///
/// Bias may be expected within the least-significant bit of the significand.
/// It is not guaranteed that exclusive limits of a range are respected; i.e.
/// when sampling the range `[a, b)` it is not guaranteed that `b` is never
/// sampled.
///
/// [`new`]: UniformSampler::new
/// [`new_inclusive`]: UniformSampler::new_inclusive
/// [`StandardUniform`]: crate::distr::StandardUniform
/// [`Uniform`]: super::Uniform
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UniformFloat<X> {
    low: X,
    scale: X,
}

macro_rules! uniform_float_impl {
    ($($meta:meta)?, $ty:ty, $uty:ident, $f_scalar:ident, $u_scalar:ident, $bits_to_discard:expr) => {
        $(#[cfg($meta)])?
        impl UniformFloat<$ty> {
            /// Construct, reducing `scale` as required to ensure that rounding
            /// can never yield values greater than `high`.
            ///
            /// Note: though it may be tempting to use a variant of this method
            /// to ensure that samples from `[low, high)` are always strictly
            /// less than `high`, this approach may be very slow where
            /// `scale.abs()` is much smaller than `high.abs()`
            /// (example: `low=0.99999999997819644, high=1.`).
            fn new_bounded(low: $ty, high: $ty, mut scale: $ty) -> Self {
                let max_rand = <$ty>::splat(1.0 as $f_scalar - $f_scalar::EPSILON);

                loop {
                    let mask = (scale * max_rand + low).gt_mask(high);
                    if !mask.any() {
                        break;
                    }
                    scale = scale.decrease_masked(mask);
                }

                debug_assert!(<$ty>::splat(0.0).all_le(scale));

                UniformFloat { low, scale }
            }
        }

        $(#[cfg($meta)])?
        impl SampleUniform for $ty {
            type Sampler = UniformFloat<$ty>;
        }

        $(#[cfg($meta)])?
        impl UniformSampler for UniformFloat<$ty> {
            type X = $ty;

            fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                #[cfg(debug_assertions)]
                if !(low.all_finite()) || !(high.all_finite()) {
                    return Err(Error::NonFinite);
                }
                if !(low.all_lt(high)) {
                    return Err(Error::EmptyRange);
                }

                let scale = high - low;
                if !(scale.all_finite()) {
                    return Err(Error::NonFinite);
                }

                Ok(Self::new_bounded(low, high, scale))
            }

            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                #[cfg(debug_assertions)]
                if !(low.all_finite()) || !(high.all_finite()) {
                    return Err(Error::NonFinite);
                }
                if !low.all_le(high) {
                    return Err(Error::EmptyRange);
                }

                let max_rand = <$ty>::splat(1.0 as $f_scalar - $f_scalar::EPSILON);
                let scale = (high - low) / max_rand;
                if !scale.all_finite() {
                    return Err(Error::NonFinite);
                }

                Ok(Self::new_bounded(low, high, scale))
            }

            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                // Generate a value in the range [1, 2)
                let value1_2 = (rng.random::<$uty>() >> $uty::splat($bits_to_discard)).into_float_with_exponent(0);

                // Get a value in the range [0, 1) to avoid overflow when multiplying by scale
                let value0_1 = value1_2 - <$ty>::splat(1.0);

                // We don't use `f64::mul_add`, because it is not available with
                // `no_std`. Furthermore, it is slower for some targets (but
                // faster for others). However, the order of multiplication and
                // addition is important, because on some platforms (e.g. ARM)
                // it will be optimized to a single (non-FMA) instruction.
                value0_1 * self.scale + self.low
            }

            #[inline]
            fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> Result<Self::X, Error>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                Self::sample_single_inclusive(low_b, high_b, rng)
            }

            #[inline]
            fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> Result<Self::X, Error>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                #[cfg(debug_assertions)]
                if !low.all_finite() || !high.all_finite() {
                    return Err(Error::NonFinite);
                }
                if !low.all_le(high) {
                    return Err(Error::EmptyRange);
                }
                let scale = high - low;
                if !scale.all_finite() {
                    return Err(Error::NonFinite);
                }

                // Generate a value in the range [1, 2)
                let value1_2 =
                    (rng.random::<$uty>() >> $uty::splat($bits_to_discard)).into_float_with_exponent(0);

                // Get a value in the range [0, 1) to avoid overflow when multiplying by scale
                let value0_1 = value1_2 - <$ty>::splat(1.0);

                // Doing multiply before addition allows some architectures
                // to use a single instruction.
                Ok(value0_1 * scale + low)
            }
        }
    };
}

uniform_float_impl! { , f32, u32, f32, u32, 32 - 23 }
uniform_float_impl! { , f64, u64, f64, u64, 64 - 52 }

#[cfg(feature = "simd_support")]
uniform_float_impl! { feature = "simd_support", f32x2, u32x2, f32, u32, 32 - 23 }
#[cfg(feature = "simd_support")]
uniform_float_impl! { feature = "simd_support", f32x4, u32x4, f32, u32, 32 - 23 }
#[cfg(feature = "simd_support")]
uniform_float_impl! { feature = "simd_support", f32x8, u32x8, f32, u32, 32 - 23 }
#[cfg(feature = "simd_support")]
uniform_float_impl! { feature = "simd_support", f32x16, u32x16, f32, u32, 32 - 23 }

#[cfg(feature = "simd_support")]
uniform_float_impl! { feature = "simd_support", f64x2, u64x2, f64, u64, 64 - 52 }
#[cfg(feature = "simd_support")]
uniform_float_impl! { feature = "simd_support", f64x4, u64x4, f64, u64, 64 - 52 }
#[cfg(feature = "simd_support")]
uniform_float_impl! { feature = "simd_support", f64x8, u64x8, f64, u64, 64 - 52 }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distr::{utils::FloatSIMDScalarUtils, Uniform};
    use crate::rngs::mock::StepRng;

    #[test]
    #[cfg_attr(miri, ignore)] // Miri is too slow
    fn test_floats() {
        let mut rng = crate::test::rng(252);
        let mut zero_rng = StepRng::new(0, 0);
        let mut max_rng = StepRng::new(0xffff_ffff_ffff_ffff, 0);
        macro_rules! t {
            ($ty:ty, $f_scalar:ident, $bits_shifted:expr) => {{
                let v: &[($f_scalar, $f_scalar)] = &[
                    (0.0, 100.0),
                    (-1e35, -1e25),
                    (1e-35, 1e-25),
                    (-1e35, 1e35),
                    (<$f_scalar>::from_bits(0), <$f_scalar>::from_bits(3)),
                    (-<$f_scalar>::from_bits(10), -<$f_scalar>::from_bits(1)),
                    (-<$f_scalar>::from_bits(5), 0.0),
                    (-<$f_scalar>::from_bits(7), -0.0),
                    (0.1 * $f_scalar::MAX, $f_scalar::MAX),
                    (-$f_scalar::MAX * 0.2, $f_scalar::MAX * 0.7),
                ];
                for &(low_scalar, high_scalar) in v.iter() {
                    for lane in 0..<$ty>::LEN {
                        let low = <$ty>::splat(0.0 as $f_scalar).replace(lane, low_scalar);
                        let high = <$ty>::splat(1.0 as $f_scalar).replace(lane, high_scalar);
                        let my_uniform = Uniform::new(low, high).unwrap();
                        let my_incl_uniform = Uniform::new_inclusive(low, high).unwrap();
                        for _ in 0..100 {
                            let v = rng.sample(my_uniform).extract_lane(lane);
                            assert!(low_scalar <= v && v <= high_scalar);
                            let v = rng.sample(my_incl_uniform).extract_lane(lane);
                            assert!(low_scalar <= v && v <= high_scalar);
                            let v =
                                <$ty as SampleUniform>::Sampler::sample_single(low, high, &mut rng)
                                    .unwrap()
                                    .extract_lane(lane);
                            assert!(low_scalar <= v && v <= high_scalar);
                            let v = <$ty as SampleUniform>::Sampler::sample_single_inclusive(
                                low, high, &mut rng,
                            )
                            .unwrap()
                            .extract_lane(lane);
                            assert!(low_scalar <= v && v <= high_scalar);
                        }

                        assert_eq!(
                            rng.sample(Uniform::new_inclusive(low, low).unwrap())
                                .extract_lane(lane),
                            low_scalar
                        );

                        assert_eq!(zero_rng.sample(my_uniform).extract_lane(lane), low_scalar);
                        assert_eq!(
                            zero_rng.sample(my_incl_uniform).extract_lane(lane),
                            low_scalar
                        );
                        assert_eq!(
                            <$ty as SampleUniform>::Sampler::sample_single(
                                low,
                                high,
                                &mut zero_rng
                            )
                            .unwrap()
                            .extract_lane(lane),
                            low_scalar
                        );
                        assert_eq!(
                            <$ty as SampleUniform>::Sampler::sample_single_inclusive(
                                low,
                                high,
                                &mut zero_rng
                            )
                            .unwrap()
                            .extract_lane(lane),
                            low_scalar
                        );

                        assert!(max_rng.sample(my_uniform).extract_lane(lane) <= high_scalar);
                        assert!(max_rng.sample(my_incl_uniform).extract_lane(lane) <= high_scalar);
                        // sample_single cannot cope with max_rng:
                        // assert!(<$ty as SampleUniform>::Sampler
                        //     ::sample_single(low, high, &mut max_rng).unwrap()
                        //     .extract(lane) <= high_scalar);
                        assert!(
                            <$ty as SampleUniform>::Sampler::sample_single_inclusive(
                                low,
                                high,
                                &mut max_rng
                            )
                            .unwrap()
                            .extract_lane(lane)
                                <= high_scalar
                        );

                        // Don't run this test for really tiny differences between high and low
                        // since for those rounding might result in selecting high for a very
                        // long time.
                        if (high_scalar - low_scalar) > 0.0001 {
                            let mut lowering_max_rng = StepRng::new(
                                0xffff_ffff_ffff_ffff,
                                (-1i64 << $bits_shifted) as u64,
                            );
                            assert!(
                                <$ty as SampleUniform>::Sampler::sample_single(
                                    low,
                                    high,
                                    &mut lowering_max_rng
                                )
                                .unwrap()
                                .extract_lane(lane)
                                    <= high_scalar
                            );
                        }
                    }
                }

                assert_eq!(
                    rng.sample(Uniform::new_inclusive($f_scalar::MAX, $f_scalar::MAX).unwrap()),
                    $f_scalar::MAX
                );
                assert_eq!(
                    rng.sample(Uniform::new_inclusive(-$f_scalar::MAX, -$f_scalar::MAX).unwrap()),
                    -$f_scalar::MAX
                );
            }};
        }

        t!(f32, f32, 32 - 23);
        t!(f64, f64, 64 - 52);
        #[cfg(feature = "simd_support")]
        {
            t!(f32x2, f32, 32 - 23);
            t!(f32x4, f32, 32 - 23);
            t!(f32x8, f32, 32 - 23);
            t!(f32x16, f32, 32 - 23);
            t!(f64x2, f64, 64 - 52);
            t!(f64x4, f64, 64 - 52);
            t!(f64x8, f64, 64 - 52);
        }
    }

    #[test]
    fn test_float_overflow() {
        assert_eq!(Uniform::try_from(f64::MIN..f64::MAX), Err(Error::NonFinite));
    }

    #[test]
    #[should_panic]
    fn test_float_overflow_single() {
        let mut rng = crate::test::rng(252);
        rng.random_range(f64::MIN..f64::MAX);
    }

    #[test]
    #[cfg(all(feature = "std", panic = "unwind"))]
    fn test_float_assertions() {
        use super::SampleUniform;
        fn range<T: SampleUniform>(low: T, high: T) -> Result<T, Error> {
            let mut rng = crate::test::rng(253);
            T::Sampler::sample_single(low, high, &mut rng)
        }

        macro_rules! t {
            ($ty:ident, $f_scalar:ident) => {{
                let v: &[($f_scalar, $f_scalar)] = &[
                    ($f_scalar::NAN, 0.0),
                    (1.0, $f_scalar::NAN),
                    ($f_scalar::NAN, $f_scalar::NAN),
                    (1.0, 0.5),
                    ($f_scalar::MAX, -$f_scalar::MAX),
                    ($f_scalar::INFINITY, $f_scalar::INFINITY),
                    ($f_scalar::NEG_INFINITY, $f_scalar::NEG_INFINITY),
                    ($f_scalar::NEG_INFINITY, 5.0),
                    (5.0, $f_scalar::INFINITY),
                    ($f_scalar::NAN, $f_scalar::INFINITY),
                    ($f_scalar::NEG_INFINITY, $f_scalar::NAN),
                    ($f_scalar::NEG_INFINITY, $f_scalar::INFINITY),
                ];
                for &(low_scalar, high_scalar) in v.iter() {
                    for lane in 0..<$ty>::LEN {
                        let low = <$ty>::splat(0.0 as $f_scalar).replace(lane, low_scalar);
                        let high = <$ty>::splat(1.0 as $f_scalar).replace(lane, high_scalar);
                        assert!(range(low, high).is_err());
                        assert!(Uniform::new(low, high).is_err());
                        assert!(Uniform::new_inclusive(low, high).is_err());
                        assert!(Uniform::new(low, low).is_err());
                    }
                }
            }};
        }

        t!(f32, f32);
        t!(f64, f64);
        #[cfg(feature = "simd_support")]
        {
            t!(f32x2, f32);
            t!(f32x4, f32);
            t!(f32x8, f32);
            t!(f32x16, f32);
            t!(f64x2, f64);
            t!(f64x4, f64);
            t!(f64x8, f64);
        }
    }

    #[test]
    fn test_uniform_from_std_range() {
        let r = Uniform::try_from(2.0f64..7.0).unwrap();
        assert_eq!(r.0.low, 2.0);
        assert_eq!(r.0.scale, 5.0);
    }

    #[test]
    fn test_uniform_from_std_range_bad_limits() {
        #![allow(clippy::reversed_empty_ranges)]
        assert!(Uniform::try_from(100.0..10.0).is_err());
        assert!(Uniform::try_from(100.0..100.0).is_err());
    }

    #[test]
    fn test_uniform_from_std_range_inclusive() {
        let r = Uniform::try_from(2.0f64..=7.0).unwrap();
        assert_eq!(r.0.low, 2.0);
        assert!(r.0.scale > 5.0);
        assert!(r.0.scale < 5.0 + 1e-14);
    }

    #[test]
    fn test_uniform_from_std_range_inclusive_bad_limits() {
        #![allow(clippy::reversed_empty_ranges)]
        assert!(Uniform::try_from(100.0..=10.0).is_err());
        assert!(Uniform::try_from(100.0..=99.0).is_err());
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::cmp::Eq;
	use rand_core::RngCore;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut alphanumeric_0: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::default();
    let mut alphanumeric_0_ref_0: &crate::distr::other::Alphanumeric = &mut alphanumeric_0;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut result_0: std::result::Result<crate::distr::uniform::int::UniformInt<u32>, distr::uniform::Error> = std::result::Result::Err(error_1);
    let mut error_2: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut result_1: std::result::Result<crate::distr::uniform::int::UniformInt<u32>, distr::uniform::Error> = std::result::Result::Err(error_2);
    let mut uniformfloat_0: crate::distr::uniform::float::UniformFloat<std::result::Result<crate::distr::uniform::int::UniformInt<u32>, distr::uniform::Error>> = crate::distr::uniform::float::UniformFloat {low: result_1, scale: result_0};
    let mut uniformfloat_0_ref_0: &crate::distr::uniform::float::UniformFloat<std::result::Result<crate::distr::uniform::int::UniformInt<u32>, distr::uniform::Error>> = &mut uniformfloat_0;
    let mut error_3: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_3_ref_0: &distr::uniform::Error = &mut error_3;
    let mut error_4: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_4_ref_0: &distr::uniform::Error = &mut error_4;
    let mut u32_0: u32 = 9137u32;
    let mut u64_0: u64 = 8674u64;
    let mut u64_1: u64 = 9590u64;
    let mut f64_0: f64 = 1697.782078f64;
    let mut u64_2: u64 = 18u64;
    let mut u32_1: u32 = 7011u32;
    let mut u64_3: u64 = 1716u64;
    let mut alphabetic_0: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
    let mut alphabetic_0_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_0;
    let mut bool_0: bool = crate::random_bool(f64_0);
    let mut error_5: distr::uniform::Error = crate::distr::uniform::Error::clone(error_4_ref_0);
    let mut error_5_ref_0: &distr::uniform::Error = &mut error_5;
    let mut error_6: distr::uniform::Error = crate::distr::uniform::Error::clone(error_0_ref_0);
    let mut error_6_ref_0: &distr::uniform::Error = &mut error_6;
    let mut tuple_0: () = crate::distr::uniform::Error::assert_receiver_is_total_eq(error_6_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_2() {
//     rusty_monitor::set_test_id(2);
//     let mut alphanumeric_0: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::default();
//     let mut alphanumeric_0_ref_0: &crate::distr::other::Alphanumeric = &mut alphanumeric_0;
//     let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
//     let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
//     let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
//     let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
//     let mut u64_0: u64 = 1855u64;
//     let mut u64_1: u64 = 9302u64;
//     let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
//     let mut steprng_0_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_0;
//     let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
//     let mut error_1_ref_0: &distr::uniform::Error = &mut error_1;
//     let mut f32_0: f32 = -13885.190257f32;
//     let mut f32_1: f32 = 6633.336860f32;
//     let mut f32_2: f32 = 4495.185108f32;
//     let mut uniformfloat_0: crate::distr::uniform::float::UniformFloat<f32> = crate::distr::uniform::float::UniformFloat::new_bounded(f32_2, f32_1, f32_0);
//     let mut tuple_0: () = crate::distr::uniform::Error::assert_receiver_is_total_eq(error_1_ref_0);
//     let mut standarduniform_0: crate::distr::StandardUniform = crate::distr::StandardUniform::default();
//     let mut uniformfloat_0_ref_0: &crate::distr::uniform::float::UniformFloat<i8> = &mut uniformfloat_0;
//     let mut uniformfloat_1: crate::distr::uniform::float::UniformFloat<i8> = crate::distr::uniform::float::UniformFloat::clone(uniformfloat_0_ref_0);
//     let mut alphanumeric_1: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::default();
//     let mut error_2: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
//     let mut error_2_ref_0: &distr::uniform::Error = &mut error_2;
//     let mut tuple_1: () = crate::distr::uniform::Error::assert_receiver_is_total_eq(error_2_ref_0);
//     let mut alphanumeric_1_ref_0: &crate::distr::other::Alphanumeric = &mut alphanumeric_1;
//     let mut alphanumeric_2: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::clone(alphanumeric_1_ref_0);
//     let mut error_3: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
//     let mut u32_0: u32 = crate::rngs::thread::ThreadRng::next_u32(threadrng_0_ref_0);
//     let mut uniformfloat_1_ref_0: &crate::distr::uniform::float::UniformFloat<i8> = &mut uniformfloat_1;
//     let mut uniformfloat_2: crate::distr::uniform::float::UniformFloat<i8> = crate::distr::uniform::float::UniformFloat::clone(uniformfloat_1_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_13() {
//     rusty_monitor::set_test_id(13);
//     let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::default();
//     let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
//     let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
//     let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
//     let mut bernoullierror_1: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
//     let mut bernoullierror_1_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_1;
//     let mut bernoullierror_2: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
//     let mut bernoullierror_2_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_2;
//     let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::default();
//     let mut threadrng_1_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_1;
//     let mut bernoullierror_3: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
//     let mut bernoullierror_3_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_3;
//     let mut bernoullierror_4: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
//     let mut bernoullierror_4_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_4;
//     let mut u16_0: u16 = 8272u16;
//     let mut u16_1: u16 = 4773u16;
//     let mut uniformfloat_0: crate::distr::uniform::float::UniformFloat<u16> = crate::distr::uniform::float::UniformFloat {low: u16_1, scale: u16_0};
//     let mut uniformfloat_0_ref_0: &crate::distr::uniform::float::UniformFloat<u16> = &mut uniformfloat_0;
//     let mut f64_0: f64 = -12426.064032f64;
//     let mut f64_1: f64 = 9193.210655f64;
//     let mut f64_2: f64 = 620.184791f64;
//     let mut uniformfloat_1: crate::distr::uniform::float::UniformFloat<f64> = crate::distr::uniform::float::UniformFloat::new_bounded(f64_2, f64_1, f64_0);
//     let mut uniformfloat_1_ref_0: &crate::distr::uniform::float::UniformFloat<u16> = &mut uniformfloat_1;
//     let mut bool_0: bool = crate::distr::uniform::float::UniformFloat::eq(uniformfloat_1_ref_0, uniformfloat_0_ref_0);
//     let mut bernoullierror_5: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
//     let mut bool_1: bool = crate::distr::bernoulli::BernoulliError::eq(bernoullierror_4_ref_0, bernoullierror_3_ref_0);
//     let mut bernoullierror_5_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_5;
//     let mut bernoullierror_6: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::clone(bernoullierror_5_ref_0);
//     let mut tuple_0: () = crate::distr::bernoulli::BernoulliError::assert_receiver_is_total_eq(bernoullierror_2_ref_0);
//     let mut bernoullierror_7: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::clone(bernoullierror_0_ref_0);
//     let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
//     let mut u64_0: u64 = crate::rngs::thread::ThreadRng::next_u64(threadrng_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_24() {
//     rusty_monitor::set_test_id(24);
//     let mut f64_0: f64 = 5090.171470f64;
//     let mut result_0: std::result::Result<crate::distr::bernoulli::Bernoulli, distr::bernoulli::BernoulliError> = crate::distr::bernoulli::Bernoulli::new(f64_0);
//     let mut f64_1: f64 = -14554.540504f64;
//     let mut result_1: std::result::Result<crate::distr::bernoulli::Bernoulli, distr::bernoulli::BernoulliError> = crate::distr::bernoulli::Bernoulli::new(f64_1);
//     let mut uniformfloat_0: crate::distr::uniform::float::UniformFloat<std::result::Result<crate::distr::bernoulli::Bernoulli, distr::bernoulli::BernoulliError>> = crate::distr::uniform::float::UniformFloat {low: result_1, scale: result_0};
//     let mut uniformfloat_0_ref_0: &crate::distr::uniform::float::UniformFloat<std::result::Result<crate::distr::bernoulli::Bernoulli, distr::bernoulli::BernoulliError>> = &mut uniformfloat_0;
//     let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
//     let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
//     let mut alphabetic_0: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
//     let mut alphabetic_0_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_0;
//     let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
//     let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
//     let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
//     let mut error_1_ref_0: &distr::weighted::Error = &mut error_1;
//     let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
//     let mut error_2_ref_0: &distr::weighted::Error = &mut error_2;
//     let mut bool_0: bool = true;
//     let mut bool_1: bool = true;
//     let mut uniformfloat_1: crate::distr::uniform::float::UniformFloat<bool> = crate::distr::uniform::float::UniformFloat {low: bool_1, scale: bool_0};
//     let mut uniformfloat_1_ref_0: &crate::distr::uniform::float::UniformFloat<bool> = &mut uniformfloat_1;
//     let mut openclosed01_0: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::default();
//     let mut openclosed01_0_ref_0: &crate::distr::float::OpenClosed01 = &mut openclosed01_0;
//     let mut f32_0: f32 = 3764.174794f32;
//     let mut f32_1: f32 = -13174.912700f32;
//     let mut f32_2: f32 = -4780.166989f32;
//     let mut uniformfloat_2: crate::distr::uniform::float::UniformFloat<f32> = crate::distr::uniform::float::UniformFloat::new_bounded(f32_2, f32_1, f32_0);
//     let mut openclosed01_1: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::clone(openclosed01_0_ref_0);
//     let mut uniformfloat_2_ref_0: &crate::distr::uniform::float::UniformFloat<bool> = &mut uniformfloat_2;
//     let mut bool_2: bool = crate::distr::uniform::float::UniformFloat::eq(uniformfloat_2_ref_0, uniformfloat_1_ref_0);
//     let mut bool_3: bool = crate::distr::weighted::Error::eq(error_2_ref_0, error_1_ref_0);
//     let mut error_3: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
//     let mut error_3_ref_0: &distr::weighted::Error = &mut error_3;
//     let mut openclosed01_1_ref_0: &crate::distr::float::OpenClosed01 = &mut openclosed01_1;
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_30() {
//     rusty_monitor::set_test_id(30);
//     let mut u128_0: u128 = 5615u128;
//     let mut u128_1: u128 = 6939u128;
//     let mut uniformfloat_0: crate::distr::uniform::float::UniformFloat<u128> = crate::distr::uniform::float::UniformFloat {low: u128_1, scale: u128_0};
//     let mut uniformfloat_0_ref_0: &crate::distr::uniform::float::UniformFloat<u128> = &mut uniformfloat_0;
//     let mut isize_0: isize = -9827isize;
//     let mut isize_1: isize = 2392isize;
//     let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
//     let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
//     let mut bernoullierror_1: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
//     let mut bernoullierror_1_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_1;
//     let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
//     let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
//     let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
//     let mut result_0: std::result::Result<crate::distr::uniform::int::UniformInt<u64>, distr::uniform::Error> = std::result::Result::Err(error_0);
//     let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
//     let mut result_1: std::result::Result<crate::distr::uniform::int::UniformInt<u64>, distr::uniform::Error> = std::result::Result::Err(error_1);
//     let mut uniformfloat_1: crate::distr::uniform::float::UniformFloat<std::result::Result<crate::distr::uniform::int::UniformInt<u64>, distr::uniform::Error>> = crate::distr::uniform::float::UniformFloat {low: result_1, scale: result_0};
//     let mut uniformfloat_1_ref_0: &crate::distr::uniform::float::UniformFloat<std::result::Result<crate::distr::uniform::int::UniformInt<u64>, distr::uniform::Error>> = &mut uniformfloat_1;
//     let mut f64_0: f64 = 2050.457810f64;
//     let mut f64_1: f64 = -9713.427126f64;
//     let mut f64_2: f64 = -12142.445071f64;
//     let mut isize_2: isize = 12172isize;
//     let mut isize_3: isize = 8901isize;
//     let mut uniformfloat_2: crate::distr::uniform::float::UniformFloat<isize> = crate::distr::uniform::float::UniformFloat {low: isize_3, scale: isize_2};
//     let mut uniformfloat_2_ref_0: &crate::distr::uniform::float::UniformFloat<isize> = &mut uniformfloat_2;
//     let mut uniformfloat_3: crate::distr::uniform::float::UniformFloat<isize> = crate::distr::uniform::float::UniformFloat::clone(uniformfloat_2_ref_0);
//     let mut uniformfloat_4: crate::distr::uniform::float::UniformFloat<f64> = crate::distr::uniform::float::UniformFloat::new_bounded(f64_2, f64_1, f64_0);
//     let mut error_2: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
//     let mut error_3: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
//     let mut uniformfloat_5: crate::distr::uniform::float::UniformFloat<isize> = crate::distr::uniform::float::UniformFloat {low: isize_1, scale: isize_0};
//     let mut uniformfloat_4_ref_0: &crate::distr::uniform::float::UniformFloat<u128> = &mut uniformfloat_4;
//     let mut bool_0: bool = crate::distr::uniform::float::UniformFloat::eq(uniformfloat_4_ref_0, uniformfloat_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut alphabetic_0: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
    let mut alphabetic_0_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_0;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut result_0: std::result::Result<u64, distr::uniform::Error> = std::result::Result::Err(error_1);
    let mut u64_0: u64 = 5020u64;
    let mut result_1: std::result::Result<u64, distr::uniform::Error> = std::result::Result::Ok(u64_0);
    let mut uniformfloat_0: crate::distr::uniform::float::UniformFloat<std::result::Result<u64, distr::uniform::Error>> = crate::distr::uniform::float::UniformFloat {low: result_1, scale: result_0};
    let mut uniformfloat_0_ref_0: &crate::distr::uniform::float::UniformFloat<std::result::Result<u64, distr::uniform::Error>> = &mut uniformfloat_0;
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut error_2_ref_0: &distr::weighted::Error = &mut error_2;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut bernoullierror_1: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_1_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_1;
    let mut bernoullierror_2: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_2_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_2;
    let mut u32_0: u32 = 3590u32;
    let mut u32_1: u32 = 3222u32;
    let mut u32_2: u32 = 9191u32;
    let mut bool_0: bool = crate::distr::bernoulli::BernoulliError::eq(bernoullierror_2_ref_0, bernoullierror_1_ref_0);
    let mut error_3: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut error_3_ref_0: &distr::weighted::Error = &mut error_3;
    let mut bool_1: bool = crate::distr::weighted::Error::eq(error_3_ref_0, error_2_ref_0);
    let mut error_4: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_4_ref_0: &distr::uniform::Error = &mut error_4;
    let mut bool_2: bool = crate::distr::uniform::Error::eq(error_4_ref_0, error_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_40() {
//     rusty_monitor::set_test_id(40);
//     let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
//     let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
//     let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
//     let mut error_1_ref_0: &distr::uniform::Error = &mut error_1;
//     let mut error_2: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
//     let mut error_2_ref_0: &distr::uniform::Error = &mut error_2;
//     let mut f32_0: f32 = 15022.853410f32;
//     let mut f32_1: f32 = -10290.673888f32;
//     let mut f32_2: f32 = 21105.229414f32;
//     let mut error_3: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
//     let mut error_3_ref_0: &distr::weighted::Error = &mut error_3;
//     let mut error_4: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
//     let mut error_4_ref_0: &distr::weighted::Error = &mut error_4;
//     let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::default();
//     let mut threadrng_0_ref_0: &crate::rngs::thread::ThreadRng = &mut threadrng_0;
//     let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::clone(threadrng_0_ref_0);
//     let mut bool_0: bool = crate::distr::weighted::Error::eq(error_4_ref_0, error_3_ref_0);
//     let mut alphanumeric_0: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::default();
//     let mut threadrng_1_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_1;
//     let mut u64_0: u64 = crate::rngs::thread::ThreadRng::next_u64(threadrng_1_ref_0);
//     let mut openclosed01_0: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::default();
//     let mut uniformfloat_0: crate::distr::uniform::float::UniformFloat<f32> = crate::distr::uniform::float::UniformFloat::new_bounded(f32_2, f32_1, f32_0);
//     let mut tuple_0: () = crate::distr::uniform::Error::assert_receiver_is_total_eq(error_2_ref_0);
//     let mut tuple_1: () = crate::distr::uniform::Error::assert_receiver_is_total_eq(error_1_ref_0);
//     let mut openclosed01_0_ref_0: &crate::distr::float::OpenClosed01 = &mut openclosed01_0;
//     let mut error_5: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
//     let mut error_5_ref_0: &distr::weighted::Error = &mut error_5;
//     let mut alphanumeric_0_ref_0: &crate::distr::other::Alphanumeric = &mut alphanumeric_0;
//     let mut alphanumeric_1: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::clone(alphanumeric_0_ref_0);
//     let mut alphabetic_0: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
    let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut error_1_ref_0: &distr::weighted::Error = &mut error_1;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::Overflow;
    let mut error_2_ref_0: &distr::weighted::Error = &mut error_2;
    let mut u64_0: u64 = 3627u64;
    let mut u64_1: u64 = 7213u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_0;
    let mut f32_0: f32 = 10418.345610f32;
    let mut f32_1: f32 = -3117.089375f32;
    let mut uniformfloat_0: crate::distr::uniform::float::UniformFloat<f32> = crate::distr::uniform::float::UniformFloat {low: f32_1, scale: f32_0};
    let mut uniformfloat_0_ref_0: &crate::distr::uniform::float::UniformFloat<f32> = &mut uniformfloat_0;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut f32_2: f32 = -9414.470614f32;
    let mut f32_3: f32 = -1818.976430f32;
    let mut uniformfloat_1: crate::distr::uniform::float::UniformFloat<f32> = crate::distr::uniform::float::UniformFloat {low: f32_3, scale: f32_2};
    let mut uniformfloat_1_ref_0: &crate::distr::uniform::float::UniformFloat<f32> = &mut uniformfloat_1;
    let mut uniformfloat_2: crate::distr::uniform::float::UniformFloat<f32> = crate::distr::uniform::float::UniformFloat::clone(uniformfloat_1_ref_0);
    let mut tuple_0: () = crate::distr::bernoulli::BernoulliError::assert_receiver_is_total_eq(bernoullierror_0_ref_0);
    let mut uniformfloat_2_ref_0: &crate::distr::uniform::float::UniformFloat<f32> = &mut uniformfloat_2;
    let mut bool_0: bool = crate::distr::uniform::float::UniformFloat::eq(uniformfloat_2_ref_0, uniformfloat_0_ref_0);
    let mut error_3: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut error_3_ref_0: &distr::weighted::Error = &mut error_3;
    let mut bool_1: bool = crate::distr::weighted::Error::eq(error_3_ref_0, error_2_ref_0);
    let mut u32_0: u32 = crate::rngs::thread::ThreadRng::next_u32(threadrng_0_ref_0);
    let mut tuple_1: () = crate::distr::weighted::Error::assert_receiver_is_total_eq(error_1_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut i64_0: i64 = 6118i64;
    let mut result_0: std::result::Result<i64, distr::uniform::Error> = std::result::Result::Ok(i64_0);
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut result_1: std::result::Result<i64, distr::uniform::Error> = std::result::Result::Err(error_0);
    let mut uniformfloat_0: crate::distr::uniform::float::UniformFloat<std::result::Result<i64, distr::uniform::Error>> = crate::distr::uniform::float::UniformFloat {low: result_1, scale: result_0};
    let mut uniformfloat_0_ref_0: &crate::distr::uniform::float::UniformFloat<std::result::Result<i64, distr::uniform::Error>> = &mut uniformfloat_0;
    let mut f32_0: f32 = -23219.593036f32;
    let mut f32_1: f32 = 7670.486647f32;
    let mut uniformfloat_1: crate::distr::uniform::float::UniformFloat<f32> = crate::distr::uniform::float::UniformFloat {low: f32_1, scale: f32_0};
    let mut uniformfloat_1_ref_0: &crate::distr::uniform::float::UniformFloat<f32> = &mut uniformfloat_1;
    let mut isize_0: isize = -13073isize;
    let mut isize_1: isize = 3618isize;
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_1_ref_0: &distr::uniform::Error = &mut error_1;
    let mut u64_0: u64 = 8611u64;
    let mut u32_0: u32 = 43u32;
    let mut u64_1: u64 = 5128u64;
    let mut u32_1: u32 = 2859u32;
    let mut u32_2: u32 = 9541u32;
    let mut u64_2: u64 = 1649u64;
    let mut u64_3: u64 = 4032u64;
    let mut error_2: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_2_ref_0: &distr::uniform::Error = &mut error_2;
    let mut error_3: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_3_ref_0: &distr::uniform::Error = &mut error_3;
    let mut error_4: distr::uniform::Error = crate::distr::uniform::Error::clone(error_3_ref_0);
    let mut error_4_ref_0: &distr::uniform::Error = &mut error_4;
    let mut bool_0: bool = crate::distr::uniform::Error::eq(error_4_ref_0, error_2_ref_0);
    let mut error_5: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_5_ref_0: &distr::uniform::Error = &mut error_5;
    let mut bool_1: bool = crate::distr::uniform::Error::eq(error_5_ref_0, error_1_ref_0);
    let mut uniformfloat_2: crate::distr::uniform::float::UniformFloat<f32> = crate::distr::uniform::float::UniformFloat::clone(uniformfloat_1_ref_0);
    panic!("From RustyUnit with love");
}
}