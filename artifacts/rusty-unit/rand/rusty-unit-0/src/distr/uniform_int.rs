// Copyright 2018-2020 Developers of the Rand project.
// Copyright 2017 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! `UniformInt` implementation

use super::{Error, SampleBorrow, SampleUniform, UniformSampler};
use crate::distr::utils::WideningMultiply;
#[cfg(feature = "simd_support")]
use crate::distr::{Distribution, StandardUniform};
use crate::Rng;

#[cfg(feature = "simd_support")]
use core::simd::prelude::*;
#[cfg(feature = "simd_support")]
use core::simd::{LaneCount, SupportedLaneCount};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The back-end implementing [`UniformSampler`] for integer types.
///
/// Unless you are implementing [`UniformSampler`] for your own type, this type
/// should not be used directly, use [`Uniform`] instead.
///
/// # Implementation notes
///
/// For simplicity, we use the same generic struct `UniformInt<X>` for all
/// integer types `X`. This gives us only one field type, `X`; to store unsigned
/// values of this size, we take use the fact that these conversions are no-ops.
///
/// For a closed range, the number of possible numbers we should generate is
/// `range = (high - low + 1)`. To avoid bias, we must ensure that the size of
/// our sample space, `zone`, is a multiple of `range`; other values must be
/// rejected (by replacing with a new random sample).
///
/// As a special case, we use `range = 0` to represent the full range of the
/// result type (i.e. for `new_inclusive($ty::MIN, $ty::MAX)`).
///
/// The optimum `zone` is the largest product of `range` which fits in our
/// (unsigned) target type. We calculate this by calculating how many numbers we
/// must reject: `reject = (MAX + 1) % range = (MAX - range + 1) % range`. Any (large)
/// product of `range` will suffice, thus in `sample_single` we multiply by a
/// power of 2 via bit-shifting (faster but may cause more rejections).
///
/// The smallest integer PRNGs generate is `u32`. For 8- and 16-bit outputs we
/// use `u32` for our `zone` and samples (because it's not slower and because
/// it reduces the chance of having to reject a sample). In this case we cannot
/// store `zone` in the target type since it is too large, however we know
/// `ints_to_reject < range <= $uty::MAX`.
///
/// An alternative to using a modulus is widening multiply: After a widening
/// multiply by `range`, the result is in the high word. Then comparing the low
/// word against `zone` makes sure our distribution is uniform.
///
/// # Bias
///
/// Unless the `unbiased` feature flag is used, outputs may have a small bias.
/// In the worst case, bias affects 1 in `2^n` samples where n is
/// 56 (`i8` and `u8`), 48 (`i16` and `u16`), 96 (`i32` and `u32`), 64 (`i64`
/// and `u64`), 128 (`i128` and `u128`).
///
/// [`Uniform`]: super::Uniform
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UniformInt<X> {
    pub(super) low: X,
    pub(super) range: X,
    thresh: X, // effectively 2.pow(max(64, uty_bits)) % range
}

macro_rules! uniform_int_impl {
    ($ty:ty, $uty:ty, $sample_ty:ident) => {
        impl SampleUniform for $ty {
            type Sampler = UniformInt<$ty>;
        }

        impl UniformSampler for UniformInt<$ty> {
            // We play free and fast with unsigned vs signed here
            // (when $ty is signed), but that's fine, since the
            // contract of this macro is for $ty and $uty to be
            // "bit-equal", so casting between them is a no-op.

            type X = $ty;

            #[inline] // if the range is constant, this helps LLVM to do the
                      // calculations at compile-time.
            fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                if !(low < high) {
                    return Err(Error::EmptyRange);
                }
                UniformSampler::new_inclusive(low, high - 1)
            }

            #[inline] // if the range is constant, this helps LLVM to do the
                      // calculations at compile-time.
            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                if !(low <= high) {
                    return Err(Error::EmptyRange);
                }

                let range = high.wrapping_sub(low).wrapping_add(1) as $uty;
                let thresh = if range > 0 {
                    let range = $sample_ty::from(range);
                    (range.wrapping_neg() % range)
                } else {
                    0
                };

                Ok(UniformInt {
                    low,
                    range: range as $ty,           // type: $uty
                    thresh: thresh as $uty as $ty, // type: $sample_ty
                })
            }

            /// Sample from distribution, Lemire's method, unbiased
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                let range = self.range as $uty as $sample_ty;
                if range == 0 {
                    return rng.random();
                }

                let thresh = self.thresh as $uty as $sample_ty;
                let hi = loop {
                    let (hi, lo) = rng.random::<$sample_ty>().wmul(range);
                    if lo >= thresh {
                        break hi;
                    }
                };
                self.low.wrapping_add(hi as $ty)
            }

            #[inline]
            fn sample_single<R: Rng + ?Sized, B1, B2>(
                low_b: B1,
                high_b: B2,
                rng: &mut R,
            ) -> Result<Self::X, Error>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                if !(low < high) {
                    return Err(Error::EmptyRange);
                }
                Self::sample_single_inclusive(low, high - 1, rng)
            }

            /// Sample single value, Canon's method, biased
            ///
            /// In the worst case, bias affects 1 in `2^n` samples where n is
            /// 56 (`i8`), 48 (`i16`), 96 (`i32`), 64 (`i64`), 128 (`i128`).
            #[cfg(not(feature = "unbiased"))]
            #[inline]
            fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(
                low_b: B1,
                high_b: B2,
                rng: &mut R,
            ) -> Result<Self::X, Error>
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                if !(low <= high) {
                    return Err(Error::EmptyRange);
                }
                let range = high.wrapping_sub(low).wrapping_add(1) as $uty as $sample_ty;
                if range == 0 {
                    // Range is MAX+1 (unrepresentable), so we need a special case
                    return Ok(rng.random());
                }

                // generate a sample using a sensible integer type
                let (mut result, lo_order) = rng.random::<$sample_ty>().wmul(range);

                // if the sample is biased...
                if lo_order > range.wrapping_neg() {
                    // ...generate a new sample to reduce bias...
                    let (new_hi_order, _) = (rng.random::<$sample_ty>()).wmul(range as $sample_ty);
                    // ... incrementing result on overflow
                    let is_overflow = lo_order.checked_add(new_hi_order as $sample_ty).is_none();
                    result += is_overflow as $sample_ty;
                }

                Ok(low.wrapping_add(result as $ty))
            }

            /// Sample single value, Canon's method, unbiased
            #[cfg(feature = "unbiased")]
            #[inline]
            fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(
                low_b: B1,
                high_b: B2,
                rng: &mut R,
            ) -> Result<Self::X, Error>
            where
                B1: SampleBorrow<$ty> + Sized,
                B2: SampleBorrow<$ty> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                if !(low <= high) {
                    return Err(Error::EmptyRange);
                }
                let range = high.wrapping_sub(low).wrapping_add(1) as $uty as $sample_ty;
                if range == 0 {
                    // Range is MAX+1 (unrepresentable), so we need a special case
                    return Ok(rng.random());
                }

                let (mut result, mut lo) = rng.random::<$sample_ty>().wmul(range);

                // In contrast to the biased sampler, we use a loop:
                while lo > range.wrapping_neg() {
                    let (new_hi, new_lo) = (rng.random::<$sample_ty>()).wmul(range);
                    match lo.checked_add(new_hi) {
                        Some(x) if x < $sample_ty::MAX => {
                            // Anything less than MAX: last term is 0
                            break;
                        }
                        None => {
                            // Overflow: last term is 1
                            result += 1;
                            break;
                        }
                        _ => {
                            // Unlikely case: must check next sample
                            lo = new_lo;
                            continue;
                        }
                    }
                }

                Ok(low.wrapping_add(result as $ty))
            }
        }
    };
}

uniform_int_impl! { i8, u8, u32 }
uniform_int_impl! { i16, u16, u32 }
uniform_int_impl! { i32, u32, u32 }
uniform_int_impl! { i64, u64, u64 }
uniform_int_impl! { i128, u128, u128 }
uniform_int_impl! { u8, u8, u32 }
uniform_int_impl! { u16, u16, u32 }
uniform_int_impl! { u32, u32, u32 }
uniform_int_impl! { u64, u64, u64 }
uniform_int_impl! { u128, u128, u128 }

#[cfg(feature = "simd_support")]
macro_rules! uniform_simd_int_impl {
    ($ty:ident, $unsigned:ident) => {
        // The "pick the largest zone that can fit in an `u32`" optimization
        // is less useful here. Multiple lanes complicate things, we don't
        // know the PRNG's minimal output size, and casting to a larger vector
        // is generally a bad idea for SIMD performance. The user can still
        // implement it manually.

        #[cfg(feature = "simd_support")]
        impl<const LANES: usize> SampleUniform for Simd<$ty, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
            Simd<$unsigned, LANES>:
                WideningMultiply<Output = (Simd<$unsigned, LANES>, Simd<$unsigned, LANES>)>,
            StandardUniform: Distribution<Simd<$unsigned, LANES>>,
        {
            type Sampler = UniformInt<Simd<$ty, LANES>>;
        }

        #[cfg(feature = "simd_support")]
        impl<const LANES: usize> UniformSampler for UniformInt<Simd<$ty, LANES>>
        where
            LaneCount<LANES>: SupportedLaneCount,
            Simd<$unsigned, LANES>:
                WideningMultiply<Output = (Simd<$unsigned, LANES>, Simd<$unsigned, LANES>)>,
            StandardUniform: Distribution<Simd<$unsigned, LANES>>,
        {
            type X = Simd<$ty, LANES>;

            #[inline] // if the range is constant, this helps LLVM to do the
                      // calculations at compile-time.
            fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
                where B1: SampleBorrow<Self::X> + Sized,
                      B2: SampleBorrow<Self::X> + Sized
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                if !(low.simd_lt(high).all()) {
                    return Err(Error::EmptyRange);
                }
                UniformSampler::new_inclusive(low, high - Simd::splat(1))
            }

            #[inline] // if the range is constant, this helps LLVM to do the
                      // calculations at compile-time.
            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
                where B1: SampleBorrow<Self::X> + Sized,
                      B2: SampleBorrow<Self::X> + Sized
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                if !(low.simd_le(high).all()) {
                    return Err(Error::EmptyRange);
                }

                // NOTE: all `Simd` operations are inherently wrapping,
                //       see https://doc.rust-lang.org/std/simd/struct.Simd.html
                let range: Simd<$unsigned, LANES> = ((high - low) + Simd::splat(1)).cast();

                // We must avoid divide-by-zero by using 0 % 1 == 0.
                let not_full_range = range.simd_gt(Simd::splat(0));
                let modulo = not_full_range.select(range, Simd::splat(1));
                let ints_to_reject = range.wrapping_neg() % modulo;

                Ok(UniformInt {
                    low,
                    // These are really $unsigned values, but store as $ty:
                    range: range.cast(),
                    thresh: ints_to_reject.cast(),
                })
            }

            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                let range: Simd<$unsigned, LANES> = self.range.cast();
                let thresh: Simd<$unsigned, LANES> = self.thresh.cast();

                // This might seem very slow, generating a whole new
                // SIMD vector for every sample rejection. For most uses
                // though, the chance of rejection is small and provides good
                // general performance. With multiple lanes, that chance is
                // multiplied. To mitigate this, we replace only the lanes of
                // the vector which fail, iteratively reducing the chance of
                // rejection. The replacement method does however add a little
                // overhead. Benchmarking or calculating probabilities might
                // reveal contexts where this replacement method is slower.
                let mut v: Simd<$unsigned, LANES> = rng.random();
                loop {
                    let (hi, lo) = v.wmul(range);
                    let mask = lo.simd_ge(thresh);
                    if mask.all() {
                        let hi: Simd<$ty, LANES> = hi.cast();
                        // wrapping addition
                        let result = self.low + hi;
                        // `select` here compiles to a blend operation
                        // When `range.eq(0).none()` the compare and blend
                        // operations are avoided.
                        let v: Simd<$ty, LANES> = v.cast();
                        return range.simd_gt(Simd::splat(0)).select(result, v);
                    }
                    // Replace only the failing lanes
                    v = mask.select(v, rng.random());
                }
            }
        }
    };

    // bulk implementation
    ($(($unsigned:ident, $signed:ident)),+) => {
        $(
            uniform_simd_int_impl!($unsigned, $unsigned);
            uniform_simd_int_impl!($signed, $unsigned);
        )+
    };
}

#[cfg(feature = "simd_support")]
uniform_simd_int_impl! { (u8, i8), (u16, i16), (u32, i32), (u64, i64) }

/// The back-end implementing [`UniformSampler`] for `usize`.
///
/// # Implementation notes
///
/// Sampling a `usize` value is usually used in relation to the length of an
/// array or other memory structure, thus it is reasonable to assume that the
/// vast majority of use-cases will have a maximum size under [`u32::MAX`].
/// In part to optimise for this use-case, but mostly to ensure that results
/// are portable across 32-bit and 64-bit architectures (as far as is possible),
/// this implementation will use 32-bit sampling when possible.
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniformUsize {
    low: usize,
    range: usize,
    thresh: usize,
    #[cfg(target_pointer_width = "64")]
    mode64: bool,
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl SampleUniform for usize {
    type Sampler = UniformUsize;
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl UniformSampler for UniformUsize {
    type X = usize;

    #[inline] // if the range is constant, this helps LLVM to do the
              // calculations at compile-time.
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if !(low < high) {
            return Err(Error::EmptyRange);
        }

        UniformSampler::new_inclusive(low, high - 1)
    }

    #[inline] // if the range is constant, this helps LLVM to do the
              // calculations at compile-time.
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if !(low <= high) {
            return Err(Error::EmptyRange);
        }

        #[cfg(target_pointer_width = "64")]
        let mode64 = high > (u32::MAX as usize);
        #[cfg(target_pointer_width = "32")]
        let mode64 = false;

        let (range, thresh);
        if cfg!(target_pointer_width = "64") && !mode64 {
            let range32 = (high as u32).wrapping_sub(low as u32).wrapping_add(1);
            range = range32 as usize;
            thresh = if range32 > 0 {
                (range32.wrapping_neg() % range32) as usize
            } else {
                0
            };
        } else {
            range = high.wrapping_sub(low).wrapping_add(1);
            thresh = if range > 0 {
                range.wrapping_neg() % range
            } else {
                0
            };
        }

        Ok(UniformUsize {
            low,
            range,
            thresh,
            #[cfg(target_pointer_width = "64")]
            mode64,
        })
    }

    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        #[cfg(target_pointer_width = "32")]
        let mode32 = true;
        #[cfg(target_pointer_width = "64")]
        let mode32 = !self.mode64;

        if mode32 {
            let range = self.range as u32;
            if range == 0 {
                return rng.random::<u32>() as usize;
            }

            let thresh = self.thresh as u32;
            let hi = loop {
                let (hi, lo) = rng.random::<u32>().wmul(range);
                if lo >= thresh {
                    break hi;
                }
            };
            self.low.wrapping_add(hi as usize)
        } else {
            let range = self.range as u64;
            if range == 0 {
                return rng.random::<u64>() as usize;
            }

            let thresh = self.thresh as u64;
            let hi = loop {
                let (hi, lo) = rng.random::<u64>().wmul(range);
                if lo >= thresh {
                    break hi;
                }
            };
            self.low.wrapping_add(hi as usize)
        }
    }

    #[inline]
    fn sample_single<R: Rng + ?Sized, B1, B2>(
        low_b: B1,
        high_b: B2,
        rng: &mut R,
    ) -> Result<Self::X, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if !(low < high) {
            return Err(Error::EmptyRange);
        }

        if cfg!(target_pointer_width = "64") && high > (u32::MAX as usize) {
            return UniformInt::<u64>::sample_single(low as u64, high as u64, rng)
                .map(|x| x as usize);
        }

        UniformInt::<u32>::sample_single(low as u32, high as u32, rng).map(|x| x as usize)
    }

    #[inline]
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(
        low_b: B1,
        high_b: B2,
        rng: &mut R,
    ) -> Result<Self::X, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if !(low <= high) {
            return Err(Error::EmptyRange);
        }

        if cfg!(target_pointer_width = "64") && high > (u32::MAX as usize) {
            return UniformInt::<u64>::sample_single_inclusive(low as u64, high as u64, rng)
                .map(|x| x as usize);
        }

        UniformInt::<u32>::sample_single_inclusive(low as u32, high as u32, rng).map(|x| x as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distr::{Distribution, Uniform};
    use core::fmt::Debug;
    use core::ops::Add;

    #[test]
    fn test_uniform_bad_limits_equal_int() {
        assert_eq!(Uniform::new(10, 10), Err(Error::EmptyRange));
    }

    #[test]
    fn test_uniform_good_limits_equal_int() {
        let mut rng = crate::test::rng(804);
        let dist = Uniform::new_inclusive(10, 10).unwrap();
        for _ in 0..20 {
            assert_eq!(rng.sample(dist), 10);
        }
    }

    #[test]
    fn test_uniform_bad_limits_flipped_int() {
        assert_eq!(Uniform::new(10, 5), Err(Error::EmptyRange));
    }

    #[test]
    #[cfg_attr(miri, ignore)] // Miri is too slow
    fn test_integers() {
        let mut rng = crate::test::rng(251);
        macro_rules! t {
            ($ty:ident, $v:expr, $le:expr, $lt:expr) => {{
                for &(low, high) in $v.iter() {
                    let my_uniform = Uniform::new(low, high).unwrap();
                    for _ in 0..1000 {
                        let v: $ty = rng.sample(my_uniform);
                        assert!($le(low, v) && $lt(v, high));
                    }

                    let my_uniform = Uniform::new_inclusive(low, high).unwrap();
                    for _ in 0..1000 {
                        let v: $ty = rng.sample(my_uniform);
                        assert!($le(low, v) && $le(v, high));
                    }

                    let my_uniform = Uniform::new(&low, high).unwrap();
                    for _ in 0..1000 {
                        let v: $ty = rng.sample(my_uniform);
                        assert!($le(low, v) && $lt(v, high));
                    }

                    let my_uniform = Uniform::new_inclusive(&low, &high).unwrap();
                    for _ in 0..1000 {
                        let v: $ty = rng.sample(my_uniform);
                        assert!($le(low, v) && $le(v, high));
                    }

                    for _ in 0..1000 {
                        let v = <$ty as SampleUniform>::Sampler::sample_single(low, high, &mut rng).unwrap();
                        assert!($le(low, v) && $lt(v, high));
                    }

                    for _ in 0..1000 {
                        let v = <$ty as SampleUniform>::Sampler::sample_single_inclusive(low, high, &mut rng).unwrap();
                        assert!($le(low, v) && $le(v, high));
                    }
                }
            }};

            // scalar bulk
            ($($ty:ident),*) => {{
                $(t!(
                    $ty,
                    [(0, 10), (10, 127), ($ty::MIN, $ty::MAX)],
                    |x, y| x <= y,
                    |x, y| x < y
                );)*
            }};

            // simd bulk
            ($($ty:ident),* => $scalar:ident) => {{
                $(t!(
                    $ty,
                    [
                        ($ty::splat(0), $ty::splat(10)),
                        ($ty::splat(10), $ty::splat(127)),
                        ($ty::splat($scalar::MIN), $ty::splat($scalar::MAX)),
                    ],
                    |x: $ty, y| x.simd_le(y).all(),
                    |x: $ty, y| x.simd_lt(y).all()
                );)*
            }};
        }
        t!(i8, i16, i32, i64, i128, u8, u16, u32, u64, usize, u128);

        #[cfg(feature = "simd_support")]
        {
            t!(u8x4, u8x8, u8x16, u8x32, u8x64 => u8);
            t!(i8x4, i8x8, i8x16, i8x32, i8x64 => i8);
            t!(u16x2, u16x4, u16x8, u16x16, u16x32 => u16);
            t!(i16x2, i16x4, i16x8, i16x16, i16x32 => i16);
            t!(u32x2, u32x4, u32x8, u32x16 => u32);
            t!(i32x2, i32x4, i32x8, i32x16 => i32);
            t!(u64x2, u64x4, u64x8 => u64);
            t!(i64x2, i64x4, i64x8 => i64);
        }
    }

    #[test]
    fn test_uniform_from_std_range() {
        let r = Uniform::try_from(2u32..7).unwrap();
        assert_eq!(r.0.low, 2);
        assert_eq!(r.0.range, 5);
    }

    #[test]
    fn test_uniform_from_std_range_bad_limits() {
        #![allow(clippy::reversed_empty_ranges)]
        assert!(Uniform::try_from(100..10).is_err());
        assert!(Uniform::try_from(100..100).is_err());
    }

    #[test]
    fn test_uniform_from_std_range_inclusive() {
        let r = Uniform::try_from(2u32..=6).unwrap();
        assert_eq!(r.0.low, 2);
        assert_eq!(r.0.range, 5);
    }

    #[test]
    fn test_uniform_from_std_range_inclusive_bad_limits() {
        #![allow(clippy::reversed_empty_ranges)]
        assert!(Uniform::try_from(100..=10).is_err());
        assert!(Uniform::try_from(100..=99).is_err());
    }

    #[test]
    fn value_stability() {
        fn test_samples<T: SampleUniform + Copy + Debug + PartialEq + Add<T>>(
            lb: T,
            ub: T,
            ub_excl: T,
            expected: &[T],
        ) where
            Uniform<T>: Distribution<T>,
        {
            let mut rng = crate::test::rng(897);
            let mut buf = [lb; 6];

            for x in &mut buf[0..3] {
                *x = T::Sampler::sample_single_inclusive(lb, ub, &mut rng).unwrap();
            }

            let distr = Uniform::new_inclusive(lb, ub).unwrap();
            for x in &mut buf[3..6] {
                *x = rng.sample(&distr);
            }
            assert_eq!(&buf, expected);

            let mut rng = crate::test::rng(897);

            for x in &mut buf[0..3] {
                *x = T::Sampler::sample_single(lb, ub_excl, &mut rng).unwrap();
            }

            let distr = Uniform::new(lb, ub_excl).unwrap();
            for x in &mut buf[3..6] {
                *x = rng.sample(&distr);
            }
            assert_eq!(&buf, expected);
        }

        test_samples(-105i8, 111, 112, &[-99, -48, 107, 72, -19, 56]);
        test_samples(2i16, 1352, 1353, &[43, 361, 1325, 1109, 539, 1005]);
        test_samples(
            -313853i32,
            13513,
            13514,
            &[-303803, -226673, 6912, -45605, -183505, -70668],
        );
        test_samples(
            131521i64,
            6542165,
            6542166,
            &[1838724, 5384489, 4893692, 3712948, 3951509, 4094926],
        );
        test_samples(
            -0x8000_0000_0000_0000_0000_0000_0000_0000i128,
            -1,
            0,
            &[
                -30725222750250982319765550926688025855,
                -75088619368053423329503924805178012357,
                -64950748766625548510467638647674468829,
                -41794017901603587121582892414659436495,
                -63623852319608406524605295913876414006,
                -17404679390297612013597359206379189023,
            ],
        );
        test_samples(11u8, 218, 219, &[17, 66, 214, 181, 93, 165]);
        test_samples(11u16, 218, 219, &[17, 66, 214, 181, 93, 165]);
        test_samples(11u32, 218, 219, &[17, 66, 214, 181, 93, 165]);
        test_samples(11u64, 218, 219, &[66, 181, 165, 127, 134, 139]);
        test_samples(11u128, 218, 219, &[181, 127, 139, 167, 141, 197]);
        test_samples(11usize, 218, 219, &[17, 66, 214, 181, 93, 165]);

        #[cfg(feature = "simd_support")]
        {
            let lb = Simd::from([11u8, 0, 128, 127]);
            let ub = Simd::from([218, 254, 254, 254]);
            let ub_excl = ub + Simd::splat(1);
            test_samples(
                lb,
                ub,
                ub_excl,
                &[
                    Simd::from([13, 5, 237, 130]),
                    Simd::from([126, 186, 149, 161]),
                    Simd::from([103, 86, 234, 252]),
                    Simd::from([35, 18, 225, 231]),
                    Simd::from([106, 153, 246, 177]),
                    Simd::from([195, 168, 149, 222]),
                ],
            );
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
	use rand_core::RngCore;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut result_0: std::result::Result<seq::index_::IndexVec, distr::weighted::Error> = std::result::Result::Err(error_0);
    let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut result_1: std::result::Result<seq::index_::IndexVec, distr::weighted::Error> = std::result::Result::Err(error_1);
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
    let mut result_2: std::result::Result<seq::index_::IndexVec, distr::weighted::Error> = std::result::Result::Err(error_2);
    let mut uniformint_0: crate::distr::uniform::int::UniformInt<std::result::Result<seq::index_::IndexVec, distr::weighted::Error>> = crate::distr::uniform::int::UniformInt {low: result_2, range: result_1, thresh: result_0};
    let mut uniformint_0_ref_0: &crate::distr::uniform::int::UniformInt<std::result::Result<seq::index_::IndexVec, distr::weighted::Error>> = &mut uniformint_0;
    let mut bool_0: bool = true;
    let mut usize_0: usize = 149usize;
    let mut usize_1: usize = 6748usize;
    let mut usize_2: usize = 585usize;
    let mut uniformusize_0: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_2, range: usize_1, thresh: usize_0, mode64: bool_0};
    let mut uniformusize_0_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_0;
    let mut bool_1: bool = true;
    let mut usize_3: usize = 130usize;
    let mut usize_4: usize = 7189usize;
    let mut usize_5: usize = 2569usize;
    let mut bool_2: bool = true;
    let mut usize_6: usize = 6775usize;
    let mut usize_7: usize = 8203usize;
    let mut usize_8: usize = 1913usize;
    let mut error_3: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_3_ref_0: &distr::uniform::Error = &mut error_3;
    let mut error_4: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut result_3: std::result::Result<crate::distr::uniform::int::UniformInt<i64>, distr::uniform::Error> = std::result::Result::Err(error_4);
    let mut f64_0: f64 = -271.506930f64;
    let mut result_4: std::result::Result<crate::distr::bernoulli::Bernoulli, distr::bernoulli::BernoulliError> = crate::distr::bernoulli::Bernoulli::new(f64_0);
    let mut uniformusize_1: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_8, range: usize_7, thresh: usize_6, mode64: bool_2};
    let mut uniformusize_2: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_5, range: usize_4, thresh: usize_3, mode64: bool_1};
    let mut uniformusize_2_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_2;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::thread_rng();
    let mut uniformusize_1_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_1;
    let mut bool_3: bool = crate::distr::uniform::int::UniformUsize::eq(uniformusize_1_ref_0, uniformusize_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3() {
    rusty_monitor::set_test_id(3);
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut result_0: std::result::Result<i32, distr::uniform::Error> = std::result::Result::Err(error_0);
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut result_1: std::result::Result<i32, distr::uniform::Error> = std::result::Result::Err(error_1);
    let mut i32_0: i32 = -245i32;
    let mut result_2: std::result::Result<i32, distr::uniform::Error> = std::result::Result::Ok(i32_0);
    let mut uniformint_0: crate::distr::uniform::int::UniformInt<std::result::Result<i32, distr::uniform::Error>> = crate::distr::uniform::int::UniformInt {low: result_2, range: result_1, thresh: result_0};
    let mut uniformint_0_ref_0: &crate::distr::uniform::int::UniformInt<std::result::Result<i32, distr::uniform::Error>> = &mut uniformint_0;
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut error_2_ref_0: &distr::weighted::Error = &mut error_2;
    let mut u32_0: u32 = 8360u32;
    let mut u32_1: u32 = 2243u32;
    let mut error_3: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_3_ref_0: &distr::uniform::Error = &mut error_3;
    let mut alphabetic_0: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
    let mut alphabetic_0_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_0;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_0_ref_0: &crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut u64_0: u64 = 2469u64;
    let mut u64_1: u64 = 4165u64;
    let mut bool_0: bool = false;
    let mut usize_0: usize = 554usize;
    let mut usize_1: usize = 9246usize;
    let mut usize_2: usize = 9815usize;
    let mut uniformusize_0: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_2, range: usize_1, thresh: usize_0, mode64: bool_0};
    let mut uniformusize_0_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_0;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut tuple_0: () = crate::distr::uniform::Error::assert_receiver_is_total_eq(error_3_ref_0);
    let mut steprng_0_ref_0: &crate::rngs::mock::StepRng = &mut steprng_0;
    let mut tuple_1: () = crate::rngs::mock::StepRng::assert_receiver_is_total_eq(steprng_0_ref_0);
    let mut bool_1: bool = crate::random_ratio(u32_1, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut u32_0: u32 = 3026u32;
    let mut u32_1: u32 = 1196u32;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
    let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut error_1_ref_0: &distr::weighted::Error = &mut error_1;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut isize_0: isize = -417isize;
    let mut isize_1: isize = -10238isize;
    let mut isize_2: isize = 9584isize;
    let mut uniformint_0: crate::distr::uniform::int::UniformInt<isize> = crate::distr::uniform::int::UniformInt {low: isize_2, range: isize_1, thresh: isize_0};
    let mut uniformint_0_ref_0: &crate::distr::uniform::int::UniformInt<isize> = &mut uniformint_0;
    let mut isize_3: isize = 3913isize;
    let mut isize_4: isize = 15920isize;
    let mut isize_5: isize = 438isize;
    let mut uniformint_1: crate::distr::uniform::int::UniformInt<isize> = crate::distr::uniform::int::UniformInt {low: isize_5, range: isize_4, thresh: isize_3};
    let mut uniformint_1_ref_0: &crate::distr::uniform::int::UniformInt<isize> = &mut uniformint_1;
    let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_1_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_1;
    let mut result_0: std::result::Result<(), rand_core::OsError> = crate::rngs::thread::ThreadRng::reseed(threadrng_1_ref_0);
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut bool_0: bool = crate::distr::uniform::int::UniformInt::eq(uniformint_1_ref_0, uniformint_0_ref_0);
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::clone(error_1_ref_0);
    let mut error_3: distr::uniform::Error = crate::distr::uniform::Error::clone(error_0_ref_0);
    let mut error_3_ref_0: &distr::uniform::Error = &mut error_3;
    let mut error_4: distr::uniform::Error = crate::distr::uniform::Error::clone(error_3_ref_0);
    let mut bool_1: bool = crate::random_ratio(u32_1, u32_0);
    let mut error_4_ref_0: &distr::uniform::Error = &mut error_4;
    let mut error_2_ref_0: &distr::weighted::Error = &mut error_2;
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5() {
    rusty_monitor::set_test_id(5);
    let mut u64_0: u64 = 1533u64;
    let mut u64_1: u64 = 1841u64;
    let mut u64_2: u64 = 8705u64;
    let mut uniformint_0: crate::distr::uniform::int::UniformInt<u64> = crate::distr::uniform::int::UniformInt {low: u64_2, range: u64_1, thresh: u64_0};
    let mut uniformint_0_ref_0: &crate::distr::uniform::int::UniformInt<u64> = &mut uniformint_0;
    let mut u64_3: u64 = 1871u64;
    let mut u64_4: u64 = 3556u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_4, u64_3);
    let mut steprng_0_ref_0: &crate::rngs::mock::StepRng = &mut steprng_0;
    let mut u64_5: u64 = 9360u64;
    let mut u64_6: u64 = 6736u64;
    let mut u64_7: u64 = 4616u64;
    let mut uniformint_1: crate::distr::uniform::int::UniformInt<u64> = crate::distr::uniform::int::UniformInt {low: u64_7, range: u64_6, thresh: u64_5};
    let mut uniformint_1_ref_0: &crate::distr::uniform::int::UniformInt<u64> = &mut uniformint_1;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut result_0: std::result::Result<crate::distr::uniform::int::UniformInt<i16>, distr::uniform::Error> = std::result::Result::Err(error_0);
    let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
    let mut result_1: std::result::Result<seq::index_::IndexVec, distr::weighted::Error> = std::result::Result::Err(error_1);
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut error_2_ref_0: &distr::weighted::Error = &mut error_2;
    let mut tuple_0: () = crate::distr::weighted::Error::assert_receiver_is_total_eq(error_2_ref_0);
    let mut indexvec_0: seq::index_::IndexVec = std::result::Result::unwrap(result_1);
    let mut bernoullierror_1: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::clone(bernoullierror_0_ref_0);
    let mut uniformint_2: crate::distr::uniform::int::UniformInt<i16> = std::result::Result::unwrap(result_0);
    let mut uniformint_3: crate::distr::uniform::int::UniformInt<u64> = crate::distr::uniform::int::UniformInt::clone(uniformint_1_ref_0);
    let mut tuple_1: () = crate::rngs::mock::StepRng::assert_receiver_is_total_eq(steprng_0_ref_0);
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::default();
    let mut uniformint_3_ref_0: &crate::distr::uniform::int::UniformInt<u64> = &mut uniformint_3;
    let mut bool_0: bool = crate::distr::uniform::int::UniformInt::eq(uniformint_3_ref_0, uniformint_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8() {
    rusty_monitor::set_test_id(8);
    let mut alphanumeric_0: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::default();
    let mut alphanumeric_0_ref_0: &crate::distr::other::Alphanumeric = &mut alphanumeric_0;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::default();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut bool_0: bool = true;
    let mut usize_0: usize = 5010usize;
    let mut usize_1: usize = 5712usize;
    let mut usize_2: usize = 3103usize;
    let mut uniformusize_0: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_2, range: usize_1, thresh: usize_0, mode64: bool_0};
    let mut uniformusize_0_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_0;
    let mut u32_0: u32 = 6834u32;
    let mut u64_0: u64 = 1337u64;
    let mut u64_1: u64 = 6512u64;
    let mut u32_1: u32 = 6779u32;
    let mut u32_2: u32 = 5369u32;
    let mut u64_2: u64 = 7652u64;
    let mut u64_3: u64 = 8929u64;
    let mut u32_3: u32 = 6745u32;
    let mut u64_4: u64 = 4570u64;
    let mut u32_4: u32 = 3530u32;
    let mut u64_5: u64 = 2463u64;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut tuple_0: () = crate::distr::bernoulli::BernoulliError::assert_receiver_is_total_eq(bernoullierror_0_ref_0);
    let mut tuple_1: () = crate::distr::uniform::int::UniformUsize::assert_receiver_is_total_eq(uniformusize_0_ref_0);
    let mut result_0: std::result::Result<(), rand_core::OsError> = crate::rngs::thread::ThreadRng::reseed(threadrng_0_ref_0);
    let mut alphanumeric_1: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::clone(alphanumeric_0_ref_0);
    let mut alphanumeric_1_ref_0: &crate::distr::other::Alphanumeric = &mut alphanumeric_1;
    let mut alphanumeric_2: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::clone(alphanumeric_1_ref_0);
    let mut alphanumeric_2_ref_0: &crate::distr::other::Alphanumeric = &mut alphanumeric_2;
    let mut alphanumeric_3: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::clone(alphanumeric_2_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10() {
    rusty_monitor::set_test_id(10);
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_1_ref_0: &distr::uniform::Error = &mut error_1;
    let mut openclosed01_0: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::default();
    let mut openclosed01_0_ref_0: &crate::distr::float::OpenClosed01 = &mut openclosed01_0;
    let mut u8_0: u8 = 40u8;
    let mut u8_1: u8 = 95u8;
    let mut u8_2: u8 = 6u8;
    let mut uniformint_0: crate::distr::uniform::int::UniformInt<u8> = crate::distr::uniform::int::UniformInt {low: u8_2, range: u8_1, thresh: u8_0};
    let mut uniformint_0_ref_0: &crate::distr::uniform::int::UniformInt<u8> = &mut uniformint_0;
    let mut error_2: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut result_0: std::result::Result<f64, distr::uniform::Error> = std::result::Result::Err(error_2);
    let mut error_3: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut result_1: std::result::Result<f64, distr::uniform::Error> = std::result::Result::Err(error_3);
    let mut f64_0: f64 = -2993.931960f64;
    let mut result_2: std::result::Result<f64, distr::uniform::Error> = std::result::Result::Ok(f64_0);
    let mut uniformint_1: crate::distr::uniform::int::UniformInt<std::result::Result<f64, distr::uniform::Error>> = crate::distr::uniform::int::UniformInt {low: result_2, range: result_1, thresh: result_0};
    let mut uniformint_1_ref_0: &crate::distr::uniform::int::UniformInt<std::result::Result<f64, distr::uniform::Error>> = &mut uniformint_1;
    let mut isize_0: isize = -4532isize;
    let mut isize_1: isize = -4271isize;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut error_4: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_4_ref_0: &distr::uniform::Error = &mut error_4;
    let mut tuple_0: () = crate::distr::uniform::int::UniformInt::assert_receiver_is_total_eq(uniformint_0_ref_0);
    let mut error_5: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_5_ref_0: &distr::uniform::Error = &mut error_5;
    let mut bool_0: bool = crate::distr::uniform::Error::eq(error_5_ref_0, error_1_ref_0);
    let mut tuple_1: () = crate::distr::weighted::Error::assert_receiver_is_total_eq(error_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut isize_0: isize = 15463isize;
    let mut isize_1: isize = -22605isize;
    let mut isize_2: isize = 9905isize;
    let mut uniformint_0: crate::distr::uniform::int::UniformInt<isize> = crate::distr::uniform::int::UniformInt {low: isize_2, range: isize_1, thresh: isize_0};
    let mut uniformint_0_ref_0: &crate::distr::uniform::int::UniformInt<isize> = &mut uniformint_0;
    let mut bool_0: bool = true;
    let mut usize_0: usize = 9657usize;
    let mut usize_1: usize = 1151usize;
    let mut usize_2: usize = 3820usize;
    let mut isize_3: isize = -23464isize;
    let mut isize_4: isize = -4951isize;
    let mut isize_5: isize = -885isize;
    let mut u64_0: u64 = 2380u64;
    let mut u32_0: u32 = 6706u32;
    let mut u64_1: u64 = 5621u64;
    let mut u32_1: u32 = 8429u32;
    let mut u32_2: u32 = 7066u32;
    let mut u64_2: u64 = 7784u64;
    let mut u64_3: u64 = 4812u64;
    let mut uniformint_1: crate::distr::uniform::int::UniformInt<isize> = crate::distr::uniform::int::UniformInt {low: isize_5, range: isize_4, thresh: isize_3};
    let mut uniformint_1_ref_0: &crate::distr::uniform::int::UniformInt<isize> = &mut uniformint_1;
    let mut uniformint_2: crate::distr::uniform::int::UniformInt<isize> = crate::distr::uniform::int::UniformInt::clone(uniformint_1_ref_0);
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
    let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::clone(error_0_ref_0);
    let mut uniformusize_0: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_2, range: usize_1, thresh: usize_0, mode64: bool_0};
    let mut error_1_ref_0: &distr::weighted::Error = &mut error_1;
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::clone(error_1_ref_0);
    let mut uniformint_2_ref_0: &crate::distr::uniform::int::UniformInt<isize> = &mut uniformint_2;
    let mut bool_1: bool = crate::distr::uniform::int::UniformInt::eq(uniformint_2_ref_0, uniformint_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut bool_0: bool = true;
    let mut usize_0: usize = 7858usize;
    let mut usize_1: usize = 7704usize;
    let mut usize_2: usize = 3212usize;
    let mut uniformusize_0: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_2, range: usize_1, thresh: usize_0, mode64: bool_0};
    let mut uniformusize_0_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_0;
    let mut bool_1: bool = false;
    let mut usize_3: usize = 5201usize;
    let mut usize_4: usize = 7980usize;
    let mut usize_5: usize = 4408usize;
    let mut uniformusize_1: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_5, range: usize_4, thresh: usize_3, mode64: bool_1};
    let mut uniformusize_1_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_1;
    let mut isize_0: isize = -1559isize;
    let mut isize_1: isize = 2755isize;
    let mut isize_2: isize = -7551isize;
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::Overflow;
    let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
    let mut u64_0: u64 = 5289u64;
    let mut u64_1: u64 = 9352u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &crate::rngs::mock::StepRng = &mut steprng_0;
    let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
    let mut error_1_ref_0: &distr::weighted::Error = &mut error_1;
    let mut bool_2: bool = false;
    let mut usize_6: usize = 6309usize;
    let mut usize_7: usize = 8900usize;
    let mut usize_8: usize = 6354usize;
    let mut isize_3: isize = -6602isize;
    let mut isize_3_ref_0: &isize = &mut isize_3;
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut error_2_ref_0: &distr::weighted::Error = &mut error_2;
    let mut tuple_0: () = crate::distr::weighted::Error::assert_receiver_is_total_eq(error_2_ref_0);
    let mut uniformusize_2: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_8, range: usize_7, thresh: usize_6, mode64: bool_2};
    let mut error_3: distr::weighted::Error = crate::distr::weighted::Error::clone(error_1_ref_0);
    let mut error_4: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut tuple_1: () = crate::rngs::mock::StepRng::assert_receiver_is_total_eq(steprng_0_ref_0);
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut uniformint_0: crate::distr::uniform::int::UniformInt<isize> = crate::distr::uniform::int::UniformInt {low: isize_2, range: isize_1, thresh: isize_0};
    let mut bool_3: bool = crate::distr::uniform::int::UniformUsize::eq(uniformusize_1_ref_0, uniformusize_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_18() {
    rusty_monitor::set_test_id(18);
    let mut alphanumeric_0: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::default();
    let mut alphanumeric_0_ref_0: &crate::distr::other::Alphanumeric = &mut alphanumeric_0;
    let mut alphanumeric_1: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::default();
    let mut alphanumeric_1_ref_0: &crate::distr::other::Alphanumeric = &mut alphanumeric_1;
    let mut bool_0: bool = false;
    let mut usize_0: usize = 4361usize;
    let mut usize_1: usize = 769usize;
    let mut usize_2: usize = 9977usize;
    let mut uniformusize_0: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_2, range: usize_1, thresh: usize_0, mode64: bool_0};
    let mut uniformusize_0_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_0;
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
    let mut bool_1: bool = true;
    let mut usize_3: usize = 5757usize;
    let mut usize_4: usize = 1082usize;
    let mut usize_5: usize = 3627usize;
    let mut uniformusize_1: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_5, range: usize_4, thresh: usize_3, mode64: bool_1};
    let mut uniformusize_1_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_1;
    let mut bool_2: bool = true;
    let mut usize_6: usize = 3986usize;
    let mut usize_7: usize = 975usize;
    let mut usize_8: usize = 3554usize;
    let mut uniformusize_2: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_8, range: usize_7, thresh: usize_6, mode64: bool_2};
    let mut uniformusize_2_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_2;
    let mut bool_3: bool = crate::distr::uniform::int::UniformUsize::eq(uniformusize_2_ref_0, uniformusize_1_ref_0);
    let mut open01_1: crate::distr::float::Open01 = crate::distr::float::Open01::clone(open01_0_ref_0);
    let mut uniformusize_3: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize::clone(uniformusize_0_ref_0);
    let mut uniformusize_3_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_3;
    let mut tuple_0: () = crate::distr::uniform::int::UniformUsize::assert_receiver_is_total_eq(uniformusize_3_ref_0);
    let mut alphanumeric_2: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::clone(alphanumeric_1_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut openclosed01_0: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::default();
    let mut openclosed01_0_ref_0: &crate::distr::float::OpenClosed01 = &mut openclosed01_0;
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::Overflow;
    let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
    let mut f64_0: f64 = -15624.063406f64;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::default();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut u32_0: u32 = 7084u32;
    let mut u32_1: u32 = 9399u32;
    let mut bool_0: bool = false;
    let mut usize_0: usize = 3772usize;
    let mut usize_1: usize = 7138usize;
    let mut usize_2: usize = 427usize;
    let mut standarduniform_0: crate::distr::StandardUniform = crate::distr::StandardUniform::default();
    let mut standarduniform_0_ref_0: &crate::distr::StandardUniform = &mut standarduniform_0;
    let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::default();
    let mut threadrng_1_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_1;
    let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut error_1_ref_0: &distr::weighted::Error = &mut error_1;
    let mut u32_2: u32 = crate::rngs::thread::ThreadRng::next_u32(threadrng_1_ref_0);
    let mut uniformusize_0: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_2, range: usize_1, thresh: usize_0, mode64: bool_0};
    let mut bool_1: bool = crate::random_ratio(u32_1, u32_0);
    let mut uniformusize_0_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_0;
    let mut uniformusize_1: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize::clone(uniformusize_0_ref_0);
    let mut alphabetic_0: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
    let mut result_0: std::result::Result<crate::distr::bernoulli::Bernoulli, distr::bernoulli::BernoulliError> = crate::distr::bernoulli::Bernoulli::new(f64_0);
    let mut tuple_0: () = crate::distr::weighted::Error::assert_receiver_is_total_eq(error_0_ref_0);
    let mut openclosed01_1: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::clone(openclosed01_0_ref_0);
    let mut openclosed01_1_ref_0: &crate::distr::float::OpenClosed01 = &mut openclosed01_1;
    let mut openclosed01_2: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::clone(openclosed01_1_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_26() {
    rusty_monitor::set_test_id(26);
    let mut bool_0: bool = true;
    let mut usize_0: usize = 3712usize;
    let mut usize_1: usize = 5380usize;
    let mut usize_2: usize = 8293usize;
    let mut uniformusize_0: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_2, range: usize_1, thresh: usize_0, mode64: bool_0};
    let mut uniformusize_0_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_0;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_1_ref_0: &distr::uniform::Error = &mut error_1;
    let mut u32_0: u32 = 2040u32;
    let mut u32_1: u32 = 957u32;
    let mut u32_2: u32 = 9727u32;
    let mut uniformint_0: crate::distr::uniform::int::UniformInt<u32> = crate::distr::uniform::int::UniformInt {low: u32_2, range: u32_1, thresh: u32_0};
    let mut u32_3: u32 = 8968u32;
    let mut u32_4: u32 = 2u32;
    let mut result_0: std::result::Result<crate::distr::bernoulli::Bernoulli, distr::bernoulli::BernoulliError> = crate::distr::bernoulli::Bernoulli::from_ratio(u32_4, u32_3);
    let mut result_0_ref_0: &std::result::Result<crate::distr::bernoulli::Bernoulli, distr::bernoulli::BernoulliError> = &mut result_0;
    let mut openclosed01_0: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::default();
    let mut openclosed01_0_ref_0: &crate::distr::float::OpenClosed01 = &mut openclosed01_0;
    let mut openclosed01_1: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::clone(openclosed01_0_ref_0);
    let mut open01_1: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut openclosed01_1_ref_0: &crate::distr::float::OpenClosed01 = &mut openclosed01_1;
    let mut tuple_0: () = crate::distr::uniform::Error::assert_receiver_is_total_eq(error_1_ref_0);
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut uniformint_0_ref_0: &crate::distr::uniform::int::UniformInt<u32> = &mut uniformint_0;
    let mut uniformint_1: crate::distr::uniform::int::UniformInt<u32> = crate::distr::uniform::int::UniformInt::clone(uniformint_0_ref_0);
    let mut error_2_ref_0: &distr::weighted::Error = &mut error_2;
    let mut bool_1: bool = crate::distr::weighted::Error::eq(error_2_ref_0, error_0_ref_0);
    let mut open01_2: crate::distr::float::Open01 = crate::distr::float::Open01::clone(open01_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_27() {
    rusty_monitor::set_test_id(27);
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::default();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut result_0: std::result::Result<crate::distr::uniform::int::UniformInt<u8>, distr::uniform::Error> = std::result::Result::Err(error_0);
    let mut u8_0: u8 = 111u8;
    let mut u8_1: u8 = 74u8;
    let mut u8_2: u8 = 77u8;
    let mut uniformint_0: crate::distr::uniform::int::UniformInt<u8> = crate::distr::uniform::int::UniformInt {low: u8_2, range: u8_1, thresh: u8_0};
    let mut result_1: std::result::Result<crate::distr::uniform::int::UniformInt<u8>, distr::uniform::Error> = std::result::Result::Ok(uniformint_0);
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut result_2: std::result::Result<crate::distr::uniform::int::UniformInt<u8>, distr::uniform::Error> = std::result::Result::Err(error_1);
    let mut uniformint_1: crate::distr::uniform::int::UniformInt<std::result::Result<crate::distr::uniform::int::UniformInt<u8>, distr::uniform::Error>> = crate::distr::uniform::int::UniformInt {low: result_2, range: result_1, thresh: result_0};
    let mut uniformint_1_ref_0: &crate::distr::uniform::int::UniformInt<std::result::Result<crate::distr::uniform::int::UniformInt<u8>, distr::uniform::Error>> = &mut uniformint_1;
    let mut i64_0: i64 = 19401i64;
    let mut i64_1: i64 = 7436i64;
    let mut i64_2: i64 = -3967i64;
    let mut uniformint_2: crate::distr::uniform::int::UniformInt<i64> = crate::distr::uniform::int::UniformInt {low: i64_2, range: i64_1, thresh: i64_0};
    let mut uniformint_2_ref_0: &crate::distr::uniform::int::UniformInt<i64> = &mut uniformint_2;
    let mut u64_0: u64 = 4322u64;
    let mut u64_1: u64 = 9893u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &crate::rngs::mock::StepRng = &mut steprng_0;
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut uniformint_3: crate::distr::uniform::int::UniformInt<i64> = crate::distr::uniform::int::UniformInt::clone(uniformint_2_ref_0);
    let mut uniformint_3_ref_0: &crate::distr::uniform::int::UniformInt<i64> = &mut uniformint_3;
    let mut tuple_0: () = crate::distr::uniform::int::UniformInt::assert_receiver_is_total_eq(uniformint_3_ref_0);
    let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
    let mut open01_1: crate::distr::float::Open01 = crate::distr::float::Open01::clone(open01_0_ref_0);
    let mut u32_0: u32 = crate::rngs::thread::ThreadRng::next_u32(threadrng_0_ref_0);
    let mut standarduniform_0: crate::distr::StandardUniform = crate::distr::StandardUniform::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut alphanumeric_0: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::default();
    let mut alphanumeric_0_ref_0: &crate::distr::other::Alphanumeric = &mut alphanumeric_0;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_1_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_1;
    let mut threadrng_2: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_2_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_2;
    let mut bool_0: bool = false;
    let mut usize_0: usize = 7441usize;
    let mut usize_1: usize = 5615usize;
    let mut usize_2: usize = 5703usize;
    let mut uniformusize_0: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_2, range: usize_1, thresh: usize_0, mode64: bool_0};
    let mut uniformusize_0_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_0;
    let mut u32_0: u32 = 6278u32;
    let mut u64_0: u64 = 6564u64;
    let mut u64_1: u64 = 2609u64;
    let mut tuple_0: () = crate::distr::uniform::int::UniformUsize::assert_receiver_is_total_eq(uniformusize_0_ref_0);
    let mut u64_2: u64 = crate::rngs::thread::ThreadRng::next_u64(threadrng_2_ref_0);
    let mut u64_3: u64 = crate::rngs::thread::ThreadRng::next_u64(threadrng_1_ref_0);
    let mut alphabetic_0: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
    let mut alphabetic_0_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_0;
    let mut alphabetic_1: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::clone(alphabetic_0_ref_0);
    let mut alphabetic_1_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_1;
    let mut alphabetic_2: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::clone(alphabetic_1_ref_0);
    let mut alphabetic_3: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
    let mut alphabetic_3_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_3;
    let mut alphabetic_2_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_2;
    let mut alphabetic_4: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::clone(alphabetic_2_ref_0);
    let mut result_0: std::result::Result<(), rand_core::OsError> = crate::rngs::thread::ThreadRng::reseed(threadrng_0_ref_0);
    let mut tuple_1: () = std::result::Result::unwrap(result_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_37() {
    rusty_monitor::set_test_id(37);
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
    let mut alphabetic_0: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
    let mut alphabetic_0_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_0;
    let mut bool_0: bool = false;
    let mut usize_0: usize = 8687usize;
    let mut usize_1: usize = 5043usize;
    let mut usize_2: usize = 4784usize;
    let mut uniformusize_0: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_2, range: usize_1, thresh: usize_0, mode64: bool_0};
    let mut uniformusize_0_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_0;
    let mut u64_0: u64 = 6728u64;
    let mut u64_1: u64 = 3052u64;
    let mut bool_1: bool = false;
    let mut usize_3: usize = 508usize;
    let mut usize_4: usize = 9787usize;
    let mut usize_5: usize = 9974usize;
    let mut uniformusize_1: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_5, range: usize_4, thresh: usize_3, mode64: bool_1};
    let mut uniformusize_1_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_1;
    let mut alphabetic_1: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
    let mut alphabetic_1_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_1;
    let mut uniformusize_2: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize::clone(uniformusize_1_ref_0);
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_0;
    let mut u64_2: u64 = crate::rngs::mock::StepRng::next_u64(steprng_0_ref_0);
    let mut uniformusize_2_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_2;
    let mut uniformusize_3: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize::clone(uniformusize_2_ref_0);
    let mut uniformusize_3_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_3;
    let mut bool_2: bool = crate::distr::uniform::int::UniformUsize::eq(uniformusize_3_ref_0, uniformusize_0_ref_0);
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::clone(error_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut isize_0: isize = 19700isize;
    let mut isize_1: isize = -6177isize;
    let mut isize_2: isize = -3499isize;
    let mut uniformint_0: crate::distr::uniform::int::UniformInt<isize> = crate::distr::uniform::int::UniformInt {low: isize_2, range: isize_1, thresh: isize_0};
    let mut uniformint_0_ref_0: &crate::distr::uniform::int::UniformInt<isize> = &mut uniformint_0;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
    let mut isize_3: isize = -14341isize;
    let mut isize_4: isize = 12164isize;
    let mut isize_5: isize = 5824isize;
    let mut uniformint_1: crate::distr::uniform::int::UniformInt<isize> = crate::distr::uniform::int::UniformInt {low: isize_5, range: isize_4, thresh: isize_3};
    let mut uniformint_1_ref_0: &crate::distr::uniform::int::UniformInt<isize> = &mut uniformint_1;
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
    let mut bool_0: bool = false;
    let mut usize_0: usize = 4274usize;
    let mut usize_1: usize = 221usize;
    let mut usize_2: usize = 8801usize;
    let mut uniformusize_0: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_2, range: usize_1, thresh: usize_0, mode64: bool_0};
    let mut uniformusize_0_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_0;
    let mut bool_1: bool = true;
    let mut usize_3: usize = 3073usize;
    let mut usize_4: usize = 6344usize;
    let mut usize_5: usize = 6300usize;
    let mut uniformusize_1: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_5, range: usize_4, thresh: usize_3, mode64: bool_1};
    let mut uniformusize_1_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_1;
    let mut bool_2: bool = crate::distr::uniform::int::UniformUsize::eq(uniformusize_1_ref_0, uniformusize_0_ref_0);
    let mut standarduniform_0: crate::distr::StandardUniform = crate::distr::StandardUniform::default();
    let mut standarduniform_0_ref_0: &crate::distr::StandardUniform = &mut standarduniform_0;
    let mut uniformint_2: crate::distr::uniform::int::UniformInt<isize> = crate::distr::uniform::int::UniformInt::clone(uniformint_1_ref_0);
    let mut tuple_0: () = crate::distr::uniform::Error::assert_receiver_is_total_eq(error_0_ref_0);
    let mut uniformint_2_ref_0: &crate::distr::uniform::int::UniformInt<isize> = &mut uniformint_2;
    let mut bool_3: bool = crate::distr::uniform::int::UniformInt::eq(uniformint_2_ref_0, uniformint_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut u32_0: u32 = 1986u32;
    let mut u32_1: u32 = 6812u32;
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::Overflow;
    let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_1_ref_0: &distr::uniform::Error = &mut error_1;
    let mut u64_0: u64 = 1977u64;
    let mut u64_1: u64 = 1657u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_0;
    let mut i64_0: i64 = 1302i64;
    let mut i64_1: i64 = -6355i64;
    let mut i64_2: i64 = 6553i64;
    let mut uniformint_0: crate::distr::uniform::int::UniformInt<i64> = crate::distr::uniform::int::UniformInt {low: i64_2, range: i64_1, thresh: i64_0};
    let mut uniformint_0_ref_0: &crate::distr::uniform::int::UniformInt<i64> = &mut uniformint_0;
    let mut i64_3: i64 = 14213i64;
    let mut i64_4: i64 = 11028i64;
    let mut i64_5: i64 = -7676i64;
    let mut uniformint_1: crate::distr::uniform::int::UniformInt<i64> = crate::distr::uniform::int::UniformInt {low: i64_5, range: i64_4, thresh: i64_3};
    let mut uniformint_1_ref_0: &crate::distr::uniform::int::UniformInt<i64> = &mut uniformint_1;
    let mut bool_0: bool = crate::distr::uniform::int::UniformInt::eq(uniformint_1_ref_0, uniformint_0_ref_0);
    let mut u32_2: u32 = crate::rngs::mock::StepRng::next_u32(steprng_0_ref_0);
    let mut tuple_0: () = crate::distr::uniform::Error::assert_receiver_is_total_eq(error_1_ref_0);
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::Overflow;
    let mut error_2_ref_0: &distr::weighted::Error = &mut error_2;
    let mut bool_1: bool = crate::distr::weighted::Error::eq(error_2_ref_0, error_0_ref_0);
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
    let mut result_0: std::result::Result<crate::distr::bernoulli::Bernoulli, distr::bernoulli::BernoulliError> = crate::distr::bernoulli::Bernoulli::from_ratio(u32_1, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut bool_0: bool = false;
    let mut usize_0: usize = 5507usize;
    let mut usize_1: usize = 4596usize;
    let mut usize_2: usize = 9374usize;
    let mut uniformusize_0: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_2, range: usize_1, thresh: usize_0, mode64: bool_0};
    let mut uniformusize_0_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_0;
    let mut bool_1: bool = true;
    let mut usize_3: usize = 6907usize;
    let mut usize_4: usize = 6635usize;
    let mut usize_5: usize = 6557usize;
    let mut uniformusize_1: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize {low: usize_5, range: usize_4, thresh: usize_3, mode64: bool_1};
    let mut uniformusize_1_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_1;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
    let mut u64_0: u64 = 3274u64;
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_1_ref_0: &distr::uniform::Error = &mut error_1;
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::Overflow;
    let mut error_3: distr::uniform::Error = crate::distr::uniform::Error::clone(error_1_ref_0);
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut u32_0: u32 = crate::rngs::thread::ThreadRng::next_u32(threadrng_0_ref_0);
    let mut error_3_ref_0: &distr::uniform::Error = &mut error_3;
    let mut error_4: distr::uniform::Error = crate::distr::uniform::Error::clone(error_3_ref_0);
    let mut error_4_ref_0: &distr::uniform::Error = &mut error_4;
    let mut bool_2: bool = crate::distr::uniform::Error::eq(error_4_ref_0, error_0_ref_0);
    let mut uniformusize_2: crate::distr::uniform::int::UniformUsize = crate::distr::uniform::int::UniformUsize::clone(uniformusize_1_ref_0);
    let mut uniformusize_2_ref_0: &crate::distr::uniform::int::UniformUsize = &mut uniformusize_2;
    let mut bool_3: bool = crate::distr::uniform::int::UniformUsize::eq(uniformusize_2_ref_0, uniformusize_0_ref_0);
    let mut error_2_ref_0: &distr::weighted::Error = &mut error_2;
    panic!("From RustyUnit with love");
}
}