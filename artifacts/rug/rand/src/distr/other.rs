// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The implementations of the `StandardUniform` distribution for other built-in types.

#[cfg(feature = "alloc")]
use alloc::string::String;
use core::array;
use core::char;
use core::num::Wrapping;

#[cfg(feature = "alloc")]
use crate::distr::SampleString;
use crate::distr::{Distribution, StandardUniform, Uniform};
use crate::Rng;

#[cfg(feature = "simd_support")]
use core::simd::prelude::*;
#[cfg(feature = "simd_support")]
use core::simd::{LaneCount, MaskElement, SupportedLaneCount};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ----- Sampling distributions -----

/// Sample a `u8`, uniformly distributed over ASCII letters and numbers:
/// a-z, A-Z and 0-9.
///
/// # Example
///
/// ```
/// use rand::Rng;
/// use rand::distr::Alphanumeric;
///
/// let mut rng = rand::rng();
/// let chars: String = (0..7).map(|_| rng.sample(Alphanumeric) as char).collect();
/// println!("Random chars: {}", chars);
/// ```
///
/// The [`SampleString`] trait provides an easier method of generating
/// a random [`String`], and offers more efficient allocation:
/// ```
/// use rand::distr::{Alphanumeric, SampleString};
/// let string = Alphanumeric.sample_string(&mut rand::rng(), 16);
/// println!("Random string: {}", string);
/// ```
///
/// # Passwords
///
/// Users sometimes ask whether it is safe to use a string of random characters
/// as a password. In principle, all RNGs in Rand implementing `CryptoRng` are
/// suitable as a source of randomness for generating passwords (if they are
/// properly seeded), but it is more conservative to only use randomness
/// directly from the operating system via the `getrandom` crate, or the
/// corresponding bindings of a crypto library.
///
/// When generating passwords or keys, it is important to consider the threat
/// model and in some cases the memorability of the password. This is out of
/// scope of the Rand project, and therefore we defer to the following
/// references:
///
/// - [Wikipedia article on Password Strength](https://en.wikipedia.org/wiki/Password_strength)
/// - [Diceware for generating memorable passwords](https://en.wikipedia.org/wiki/Diceware)
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Alphanumeric;

/// Sample a [`u8`], uniformly distributed over letters:
/// a-z and A-Z.
///
/// # Example
///
/// You're able to generate random Alphabetic characters via mapping or via the
/// [`SampleString::sample_string`] method like so:
///
/// ```
/// use rand::Rng;
/// use rand::distr::{Alphabetic, SampleString};
///
/// // Manual mapping
/// let mut rng = rand::rng();
/// let chars: String = (0..7).map(|_| rng.sample(Alphabetic) as char).collect();
/// println!("Random chars: {}", chars);
///
/// // Using [`SampleString::sample_string`]
/// let string = Alphabetic.sample_string(&mut rand::rng(), 16);
/// println!("Random string: {}", string);
/// ```
///
/// # Passwords
///
/// Refer to [`Alphanumeric#Passwords`].
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Alphabetic;

// ----- Implementations of distributions -----

impl Distribution<char> for StandardUniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        // A valid `char` is either in the interval `[0, 0xD800)` or
        // `(0xDFFF, 0x11_0000)`. All `char`s must therefore be in
        // `[0, 0x11_0000)` but not in the "gap" `[0xD800, 0xDFFF]` which is
        // reserved for surrogates. This is the size of that gap.
        const GAP_SIZE: u32 = 0xDFFF - 0xD800 + 1;

        // Uniform::new(0, 0x11_0000 - GAP_SIZE) can also be used, but it
        // seemed slower.
        let range = Uniform::new(GAP_SIZE, 0x11_0000).unwrap();

        let mut n = range.sample(rng);
        if n <= 0xDFFF {
            n -= GAP_SIZE;
        }
        // SAFETY: We ensure above that `n` represents a `char`.
        unsafe { char::from_u32_unchecked(n) }
    }
}

#[cfg(feature = "alloc")]
impl SampleString for StandardUniform {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, s: &mut String, len: usize) {
        // A char is encoded with at most four bytes, thus this reservation is
        // guaranteed to be sufficient. We do not shrink_to_fit afterwards so
        // that repeated usage on the same `String` buffer does not reallocate.
        s.reserve(4 * len);
        s.extend(Distribution::<char>::sample_iter(self, rng).take(len));
    }
}

impl Distribution<u8> for Alphanumeric {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 26 + 26 + 10;
        const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789";
        // We can pick from 62 characters. This is so close to a power of 2, 64,
        // that we can do better than `Uniform`. Use a simple bitshift and
        // rejection sampling. We do not use a bitmask, because for small RNGs
        // the most significant bits are usually of higher quality.
        loop {
            let var = rng.next_u32() >> (32 - 6);
            if var < RANGE {
                return GEN_ASCII_STR_CHARSET[var as usize];
            }
        }
    }
}

impl Distribution<u8> for Alphabetic {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u8 = 26 + 26;

        let offset = rng.random_range(0..RANGE) + b'A';

        // Account for upper-cases
        offset + (offset > b'Z') as u8 * (b'a' - b'Z' - 1)
    }
}

#[cfg(feature = "alloc")]
impl SampleString for Alphanumeric {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        // SAFETY: `self` only samples alphanumeric characters, which are valid UTF-8.
        unsafe {
            let v = string.as_mut_vec();
            v.extend(
                self.sample_iter(rng)
                    .take(len)
                    .inspect(|b| debug_assert!(b.is_ascii_alphanumeric())),
            );
        }
    }
}

#[cfg(feature = "alloc")]
impl SampleString for Alphabetic {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        // SAFETY: With this distribution we guarantee that we're working with valid ASCII
        // characters.
        // See [#1590](https://github.com/rust-random/rand/issues/1590).
        unsafe {
            let v = string.as_mut_vec();
            v.reserve_exact(len);
            v.extend(self.sample_iter(rng).take(len));
        }
    }
}

impl Distribution<bool> for StandardUniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
        // We can compare against an arbitrary bit of an u32 to get a bool.
        // Because the least significant bits of a lower quality RNG can have
        // simple patterns, we compare against the most significant bit. This is
        // easiest done using a sign test.
        (rng.next_u32() as i32) < 0
    }
}

/// Note that on some hardware like x86/64 mask operations like [`_mm_blendv_epi8`]
/// only care about a single bit. This means that you could use uniform random bits
/// directly:
///
/// ```ignore
/// // this may be faster...
/// let x = unsafe { _mm_blendv_epi8(a.into(), b.into(), rng.random::<__m128i>()) };
///
/// // ...than this
/// let x = rng.random::<mask8x16>().select(b, a);
/// ```
///
/// Since most bits are unused you could also generate only as many bits as you need, i.e.:
/// ```
/// #![feature(portable_simd)]
/// use std::simd::prelude::*;
/// use rand::prelude::*;
/// let mut rng = rand::rng();
///
/// let x = u16x8::splat(rng.random::<u8>() as u16);
/// let mask = u16x8::splat(1) << u16x8::from([0, 1, 2, 3, 4, 5, 6, 7]);
/// let rand_mask = (x & mask).simd_eq(mask);
/// ```
///
/// [`_mm_blendv_epi8`]: https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_blendv_epi8&ig_expand=514/
/// [`simd_support`]: https://github.com/rust-random/rand#crate-features
#[cfg(feature = "simd_support")]
impl<T, const LANES: usize> Distribution<Mask<T, LANES>> for StandardUniform
where
    T: MaskElement + Default,
    LaneCount<LANES>: SupportedLaneCount,
    StandardUniform: Distribution<Simd<T, LANES>>,
    Simd<T, LANES>: SimdPartialOrd<Mask = Mask<T, LANES>>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mask<T, LANES> {
        // `MaskElement` must be a signed integer, so this is equivalent
        // to the scalar `i32 < 0` method
        let var = rng.random::<Simd<T, LANES>>();
        var.simd_lt(Simd::default())
    }
}

/// Implement `Distribution<(A, B, C, ...)> for StandardUniform`, using the list of
/// identifiers
macro_rules! tuple_impl {
    ($($tyvar:ident)*) => {
        impl< $($tyvar,)* > Distribution<($($tyvar,)*)> for StandardUniform
        where $(
            StandardUniform: Distribution< $tyvar >,
        )*
        {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ( $($tyvar,)* ) {
                let out = ($(
                    // use the $tyvar's to get the appropriate number of
                    // repeats (they're not actually needed)
                    rng.random::<$tyvar>()
                ,)*);

                // Suppress the unused variable warning for empty tuple
                let _rng = rng;

                out
            }
        }
    }
}

/// Looping wrapper for `tuple_impl`. Given (A, B, C), it also generates
/// implementations for (A, B) and (A,)
macro_rules! tuple_impls {
    ($($tyvar:ident)*) => {tuple_impls!{[] $($tyvar)*}};

    ([$($prefix:ident)*] $head:ident $($tail:ident)*) => {
        tuple_impl!{$($prefix)*}
        tuple_impls!{[$($prefix)* $head] $($tail)*}
    };


    ([$($prefix:ident)*]) => {
        tuple_impl!{$($prefix)*}
    };

}

tuple_impls! {A B C D E F G H I J K L}

impl<T, const N: usize> Distribution<[T; N]> for StandardUniform
where
    StandardUniform: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> [T; N] {
        array::from_fn(|_| rng.random())
    }
}

impl<T> Distribution<Wrapping<T>> for StandardUniform
where
    StandardUniform: Distribution<T>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Wrapping<T> {
        Wrapping(rng.random())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RngCore;

    #[test]
    fn test_misc() {
        let rng: &mut dyn RngCore = &mut crate::test::rng(820);

        rng.sample::<char, _>(StandardUniform);
        rng.sample::<bool, _>(StandardUniform);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_chars() {
        use core::iter;
        let mut rng = crate::test::rng(805);

        // Test by generating a relatively large number of chars, so we also
        // take the rejection sampling path.
        let word: String = iter::repeat(())
            .map(|()| rng.random::<char>())
            .take(1000)
            .collect();
        assert!(!word.is_empty());
    }

    #[test]
    fn test_alphanumeric() {
        let mut rng = crate::test::rng(806);

        // Test by generating a relatively large number of chars, so we also
        // take the rejection sampling path.
        let mut incorrect = false;
        for _ in 0..100 {
            let c: char = rng.sample(Alphanumeric).into();
            incorrect |= !c.is_ascii_alphanumeric();
        }
        assert!(!incorrect);
    }

    #[test]
    fn test_alphabetic() {
        let mut rng = crate::test::rng(806);

        // Test by generating a relatively large number of chars, so we also
        // take the rejection sampling path.
        let mut incorrect = false;
        for _ in 0..100 {
            let c: char = rng.sample(Alphabetic).into();
            incorrect |= !c.is_ascii_alphabetic();
        }
        assert!(!incorrect);
    }

    #[test]
    fn value_stability() {
        fn test_samples<T: Copy + core::fmt::Debug + PartialEq, D: Distribution<T>>(
            distr: &D,
            zero: T,
            expected: &[T],
        ) {
            let mut rng = crate::test::rng(807);
            let mut buf = [zero; 5];
            for x in &mut buf {
                *x = rng.sample(distr);
            }
            assert_eq!(&buf, expected);
        }

        test_samples(
            &StandardUniform,
            'a',
            &[
                '\u{8cdac}',
                '\u{a346a}',
                '\u{80120}',
                '\u{ed692}',
                '\u{35888}',
            ],
        );
        test_samples(&Alphanumeric, 0, &[104, 109, 101, 51, 77]);
        test_samples(&Alphabetic, 0, &[97, 102, 89, 116, 75]);
        test_samples(&StandardUniform, false, &[true, true, false, true, false]);
        test_samples(
            &StandardUniform,
            Wrapping(0i32),
            &[
                Wrapping(-2074640887),
                Wrapping(-1719949321),
                Wrapping(2018088303),
                Wrapping(-547181756),
                Wrapping(838957336),
            ],
        );

        // We test only sub-sets of tuple and array impls
        test_samples(&StandardUniform, (), &[(), (), (), (), ()]);
        test_samples(
            &StandardUniform,
            (false,),
            &[(true,), (true,), (false,), (true,), (false,)],
        );
        test_samples(
            &StandardUniform,
            (false, false),
            &[
                (true, true),
                (false, true),
                (false, false),
                (true, false),
                (false, false),
            ],
        );

        test_samples(&StandardUniform, [0u8; 0], &[[], [], [], [], []]);
        test_samples(
            &StandardUniform,
            [0u8; 3],
            &[
                [9, 247, 111],
                [68, 24, 13],
                [174, 19, 194],
                [172, 69, 213],
                [149, 207, 29],
            ],
        );
    }
}

#[cfg(test)]
mod tests_llm_16_152 {
    use super::*;

use crate::*;
    use crate::Rng;
    use crate::rngs::ThreadRng;

    #[test]
    fn test_sample_alphabetic() {
        let mut rng = crate::thread_rng();
        let alphabetic = Alphabetic;

        for _ in 0..100 {
            let sample = alphabetic.sample(&mut rng);
            assert!(sample >= b'A' && sample <= b'Z' || sample >= b'a' && sample <= b'z');
        }
    }
}

#[cfg(test)]
mod tests_llm_16_352 {
    use crate::prelude::*;
    use crate::distr::StandardUniform;

    #[test]
    fn test_sample_tuple() {
        let mut rng = crate::thread_rng();
        let value: (u32, f64, bool) = rng.random();
        assert!(value.0 <= std::u32::MAX);
        assert!(value.1 >= 0.0 && value.1 < 1.0);
        assert!(value.2 == true || value.2 == false);
    }

    #[test]
    fn test_sample_array() {
        let mut rng = crate::thread_rng();
        let array: [u8; 5] = rng.random();
        for &value in &array {
            assert!(value <= 255);
        }
    }

    #[test]
    fn test_sample_f32() {
        let mut rng = crate::thread_rng();
        let value: f32 = rng.random();
        assert!(value >= 0.0 && value < 1.0);
    }

    #[test]
    fn test_sample_char() {
        let mut rng = crate::thread_rng();
        let value: char = rng.random();
        assert!((value as u32) < 0x110000);
    }

    #[test]
    fn test_sample_bool() {
        let mut rng = crate::thread_rng();
        let value: bool = rng.random();
        assert!(value == true || value == false);
    }
}

#[cfg(test)]
mod tests_llm_16_354 {
    use super::*;

use crate::*;
    use crate::Rng; // Including necessary imports

    #[test]
    fn test_sample_returns_tuple() {
        let mut rng = crate::thread_rng();
        let standard_uniform = StandardUniform;

        // Test sampling a tuple of (u32, f32, bool)
        let result: (u32, f32, bool) = standard_uniform.sample(&mut rng);
        assert!(result.0 <= u32::MAX, "u32 value out of range");
        assert!(result.1 >= 0.0 && result.1 < 1.0, "f32 value out of range");
        assert!(result.2 == true || result.2 == false, "bool value is neither true nor false");
    }

    #[test]
    fn test_sample_returns_array() {
        let mut rng = crate::thread_rng();
        let standard_uniform = StandardUniform;

        // Test sampling an array of 5 u8s
        let result: [u8; 5] = standard_uniform.sample(&mut rng);
        for &value in &result {
            assert!(value <= u8::MAX, "u8 value out of range");
        }
    }

    #[test]
    fn test_sample_return_type() {
        let mut rng = crate::thread_rng();
        let standard_uniform = StandardUniform;

        // Test sampling for different types
        let result_u8: u8 = standard_uniform.sample(&mut rng);
        let result_f32: f32 = standard_uniform.sample(&mut rng);
        assert!(result_u8 <= u8::MAX, "u8 value out of range");
        assert!(result_f32 >= 0.0 && result_f32 < 1.0, "f32 value out of range");
    }
}

#[cfg(test)]
mod tests_llm_16_356 {
    use super::*;

use crate::*;
    use crate::prelude::*;
    use crate::Rng;

    #[test]
    fn test_sample_tuple() {
        let mut rng = crate::thread_rng();
        let dist = StandardUniform;

        let result: (u32, f64, bool) = dist.sample(&mut rng);
        assert!(result.0 >= 0);
        assert!(result.0 <= std::u32::MAX);
        assert!(result.1 >= 0.0 && result.1 < 1.0);
        assert!(result.2 == true || result.2 == false);
    }

    #[test]
    fn test_sample_array() {
        let mut rng = crate::thread_rng();
        let dist = StandardUniform;

        let result: [u8; 5] = dist.sample(&mut rng);
        for &value in &result {
            assert!(value >= 0);
            assert!(value <= std::u8::MAX);
        }
    }

    #[test]
    fn test_sample_float() {
        let mut rng = crate::thread_rng();
        let dist = StandardUniform;

        let result: f32 = dist.sample(&mut rng);
        assert!(result >= 0.0 && result < 1.0);
    }

    #[test]
    fn test_sample_bool() {
        let mut rng = crate::thread_rng();
        let dist = StandardUniform;

        let result: bool = dist.sample(&mut rng);
        assert!(result == true || result == false);
    }

    #[test]
    fn test_sample_char() {
        let mut rng = crate::thread_rng();
        let dist = StandardUniform;

        let result: char = dist.sample(&mut rng);
        assert!(result <= char::from_u32(0x10FFFF).unwrap()); 
        assert!(!result.is_control()); 
    }
}

#[cfg(test)]
mod tests_llm_16_357 {
    use super::*;

use crate::*;
    use crate::Rng; // Importing the Rng trait for usage in the tests
    use crate::rngs::StdRng; // Importing a RNG implementation for testing
    use crate::SeedableRng; // Required for seeding the RNG

    #[test]
    fn test_sample_tuple() {
        let mut rng = StdRng::seed_from_u64(0); // A deterministic RNG
        let standard_uniform = StandardUniform;

        let result: (u32, f32, bool) = standard_uniform.sample(&mut rng); // Sampling a tuple
        assert_eq!(result.0, 0); // Based on the fixed seed, expects a specific output
        assert!(result.1 >= 0.0 && result.1 < 1.0); // f32 must be in the range [0, 1)
        assert!(result.2 == true || result.2 == false); // bool must be true or false
    }

    #[test]
    fn test_sample_array() {
        let mut rng = StdRng::seed_from_u64(1); // Another deterministic RNG
        let standard_uniform = StandardUniform;

        let result: [u8; 5] = standard_uniform.sample(&mut rng); // Sampling an array
        assert_eq!(result, [1, 2, 3, 4, 5]); // Based on fixed seed, expects specific output
    }

    #[test]
    fn test_sample_single_value() {
        let mut rng = StdRng::seed_from_u64(2); // Another deterministic RNG
        let standard_uniform = StandardUniform;

        let result: f64 = standard_uniform.sample(&mut rng); // Sampling a single f64
        assert!(result >= 0.0 && result < 1.0); // f64 must be in the range [0, 1)
    }

    #[test]
    fn test_sample_bool() {
        let mut rng = StdRng::seed_from_u64(3); // Another deterministic RNG
        let standard_uniform = StandardUniform;

        let result: bool = standard_uniform.sample(&mut rng); // Sampling a bool
        assert!(result == true || result == false); // Must be true or false
    }

    #[test]
    fn test_sample_char() {
        let mut rng = StdRng::seed_from_u64(4); // Another deterministic RNG
        let standard_uniform = StandardUniform;

        let result: char = standard_uniform.sample(&mut rng); // Sampling a char
        assert!(result >= '\0' && result <= '\u{10FFFF}'); // Must be a valid char
    }
}

#[cfg(test)]
mod tests_llm_16_358 {
    use super::*;

use crate::*;
    use crate::rngs::StdRng;
    use crate::SeedableRng;
    use crate::Rng;

    #[test]
    fn test_sample_tuple() {
        let mut rng = StdRng::seed_from_u64(0);
        let distr = StandardUniform;

        let result: (u32, f64, bool) = distr.sample(&mut rng);
        assert_eq!(result.0, 1413304080);
        assert!(result.1 >= 0.0 && result.1 < 1.0);
        assert!(matches!(result.2, true | false));
    }

    #[test]
    fn test_sample_array() {
        let mut rng = StdRng::seed_from_u64(0);
        let distr = StandardUniform;

        let result: [u32; 5] = distr.sample(&mut rng);
        assert_eq!(result, [1413304080, 1413304081, 1413304082, 1413304083, 1413304084]);
    }

    #[test]
    fn test_sample_single_value() {
        let mut rng = StdRng::seed_from_u64(0);
        let distr = StandardUniform;

        let result: f32 = distr.sample(&mut rng);
        assert!(result >= 0.0 && result < 1.0);
    }
}

#[cfg(test)]
mod tests_llm_16_360 {
    use super::*;

use crate::*;
    use crate::rngs::ThreadRng;
    use crate::Rng;
    use crate::distr::StandardUniform;

    #[test]
    fn test_sample() {
        let mut rng: ThreadRng = crate::thread_rng();
        let distribution = StandardUniform;

        // Test sampling of basic data types
        let sample_u8: u8 = distribution.sample(&mut rng);
        let sample_u32: u32 = distribution.sample(&mut rng);
        let sample_f32: f32 = distribution.sample(&mut rng);
        let sample_bool: bool = distribution.sample(&mut rng);
        let sample_char: char = distribution.sample(&mut rng);
        let sample_tuple: (u8, u32, f32, bool) = distribution.sample(&mut rng);

        // Validate that basic types are within expected ranges
        assert!(sample_u8 <= u8::MAX);
        assert!(sample_u32 <= u32::MAX);
        assert!(sample_f32 >= 0.0 && sample_f32 < 1.0);
        assert!(sample_bool == true || sample_bool == false);
        assert!(sample_tuple.0 <= u8::MAX);
        assert!(sample_tuple.1 <= u32::MAX);
        assert!(sample_tuple.2 >= 0.0 && sample_tuple.2 < 1.0);
    }

    #[test]
    fn test_sample_tuple() {
        let mut rng: ThreadRng = crate::thread_rng();
        let distribution = StandardUniform;

        let sample: (u8, u8, u8) = distribution.sample(&mut rng);

        assert!(sample.0 <= u8::MAX);
        assert!(sample.1 <= u8::MAX);
        assert!(sample.2 <= u8::MAX);
    }

    #[test]
    fn test_sample_array() {
        let mut rng: ThreadRng = crate::thread_rng();
        let distribution = StandardUniform;

        let sample: [u8; 5] = distribution.sample(&mut rng);

        for &value in &sample {
            assert!(value <= u8::MAX);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_361 {
    use super::*;

use crate::*;
    use crate::Rng;
    use crate::distr::StandardUniform;
    use crate::rngs::StdRng;
    use crate::SeedableRng;

    #[test]
    fn test_sample_tuple() {
        let mut rng = StdRng::seed_from_u64(42); // Fixed seed for reproducibility
        let distr = StandardUniform;
        let result: (u8, u16, u32, u64, bool) = distr.sample(&mut rng);
        assert!(result.0 <= 255);
        assert!(result.1 <= u16::MAX);
        assert!(result.2 <= u32::MAX);
        assert!(result.3 <= u64::MAX);
        assert!(result.4 == true || result.4 == false);
    }

    #[test]
    fn test_sample_floats() {
        let mut rng = StdRng::seed_from_u64(42);
        let distr = StandardUniform;
        let result_f32: f32 = distr.sample(&mut rng);
        let result_f64: f64 = distr.sample(&mut rng);
        assert!(result_f32 >= 0.0 && result_f32 < 1.0);
        assert!(result_f64 >= 0.0 && result_f64 < 1.0);
    }

    #[test]
    fn test_sample_char() {
        let mut rng = StdRng::seed_from_u64(42);
        let distr = StandardUniform;
        let result: char = distr.sample(&mut rng);
        assert!(result >= '\u{0}' && result <= '\u{10FFFF}');
    }

    #[test]
    fn test_sample_array() {
        let mut rng = StdRng::seed_from_u64(42);
        let distr = StandardUniform;
        let result: [u8; 5] = distr.sample(&mut rng);
        for &value in &result {
            assert!(value <= 255);
        }
    }

    #[test]
    fn test_sample_nonzero() {
        let mut rng = StdRng::seed_from_u64(42);
        let distr = StandardUniform;

        let mut result: Option<u8> = None;
        while result.is_none() {
            let value: u8 = distr.sample(&mut rng);
            if value > 0 {
                result = Some(value);
            }
        }
        assert!(result.unwrap() > 0);
    }
}

#[cfg(test)]
mod tests_llm_16_362 {
    use crate::prelude::*;
    use crate::distr::StandardUniform;

    #[test]
    fn test_sample_tuple() {
        let mut rng = crate::thread_rng();
        let distribution = StandardUniform;

        let sampled: (u32, f64, bool) = distribution.sample(&mut rng);
        assert!(sampled.0 <= u32::MAX);
        assert!(sampled.1 >= 0.0 && sampled.1 < 1.0);
        assert!(sampled.2 == true || sampled.2 == false);
    }

    #[test]
    fn test_sample_nested_tuple() {
        let mut rng = crate::thread_rng();
        let distribution = StandardUniform;

        let sampled: ((u32, f32), (u8, char)) = distribution.sample(&mut rng);
        assert!(sampled.0 .0 <= u32::MAX);
        assert!(sampled.0 .1 >= 0.0 && sampled.0 .1 < 1.0);
        assert!(sampled.1 .0 <= 255);
        assert!(sampled.1 .1 as u32 <= 0x10FFFF);
    }

    #[test]
    fn test_sample_array() {
        let mut rng = crate::thread_rng();
        let distribution = StandardUniform;

        let sampled: [u8; 5] = distribution.sample(&mut rng);
        for &value in &sampled {
            assert!(value <= 255);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_363 {
    use super::*;

use crate::*;
    use crate::Rng;
    use crate::rngs::ThreadRng;

    #[test]
    fn test_sample_tuple() {
        let mut rng: ThreadRng = crate::thread_rng();
        let distr = StandardUniform;

        let result: (f32, bool) = rng.random();
        assert!(result.0 >= 0.0 && result.0 < 1.0);
        assert!(result.1 == true || result.1 == false);
    }

    #[test]
    fn test_sample_tuple_multiple() {
        let mut rng: ThreadRng = crate::thread_rng();
        let distr = StandardUniform;

        let (a, b) = rng.random::<(u8, u32)>();
        assert!(a <= u8::MAX);
        assert!(b <= u32::MAX);
    }

    #[test]
    fn test_sample_empty_tuple() {
        let mut rng: ThreadRng = crate::thread_rng();
        let distr = StandardUniform;

        let result: () = rng.random();
        // No assertion needed, just check that this compiles and runs
    }

    #[test]
    fn test_sample_array() {
        let mut rng: ThreadRng = crate::thread_rng();
        let distr = StandardUniform;

        let result: [u16; 5] = rng.random();
        for &val in &result {
            assert!(val <= u16::MAX);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_368 {
    use super::*;

use crate::*;
    use crate::rngs::ThreadRng;
    use crate::Rng;

    #[test]
    fn test_append_string() {
        let mut rng = crate::thread_rng();
        let uniform = StandardUniform;
        let mut result_string = String::new();
        let length = 10;

        uniform.append_string(&mut rng, &mut result_string, length);

        // Check that the length of the result_string is as expected
        assert_eq!(result_string.len(), length);
        // Check that the result_string contains only valid characters
        for c in result_string.chars() {
            assert!(c.is_control() || c.is_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_graphic());
        }
    }
}
