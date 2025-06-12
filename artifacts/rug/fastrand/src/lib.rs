//! A simple and fast random number generator.
//!
//! The implementation uses [Wyrand](https://github.com/wangyi-fudan/wyhash), a simple and fast
//! generator but **not** cryptographically secure.
//!
//! # Examples
//!
//! Flip a coin:
//!
//! ```
//! if fastrand::bool() {
//!     println!("heads");
//! } else {
//!     println!("tails");
//! }
//! ```
//!
//! Generate a random `i32`:
//!
//! ```
//! let num = fastrand::i32(..);
//! ```
//!
//! Choose a random element in an array:
//!
//! ```
//! let v = vec![1, 2, 3, 4, 5];
//! let i = fastrand::usize(..v.len());
//! let elem = v[i];
//! ```
//!
//! Sample values from an array with `O(n)` complexity (`n` is the length of array):
//!
//! ```
//! fastrand::choose_multiple([1, 4, 5], 2);
//! fastrand::choose_multiple(0..20, 12);
//! ```
//!
//!
//! Shuffle an array:
//!
//! ```
//! let mut v = vec![1, 2, 3, 4, 5];
//! fastrand::shuffle(&mut v);
//! ```
//!
//! Generate a random [`Vec`] or [`String`]:
//!
//! ```
//! use std::iter::repeat_with;
//!
//! let v: Vec<i32> = repeat_with(|| fastrand::i32(..)).take(10).collect();
//! let s: String = repeat_with(fastrand::alphanumeric).take(10).collect();
//! ```
//!
//! To get reproducible results on every run, initialize the generator with a seed:
//!
//! ```
//! // Pick an arbitrary number as seed.
//! fastrand::seed(7);
//!
//! // Now this prints the same number on every run:
//! println!("{}", fastrand::u32(..));
//! ```
//!
//! To be more efficient, create a new [`Rng`] instance instead of using the thread-local
//! generator:
//!
//! ```
//! use std::iter::repeat_with;
//!
//! let mut rng = fastrand::Rng::new();
//! let mut bytes: Vec<u8> = repeat_with(|| rng.u8(..)).take(10_000).collect();
//! ```
//!
//! This crate aims to expose a core set of useful randomness primitives. For more niche algorithms,
//! consider using the [`fastrand-contrib`] crate alongside this one.
//!
//! # Features
//!
//! - `std` (enabled by default): Enables the `std` library. This is required for the global
//!   generator and global entropy. Without this feature, [`Rng`] can only be instantiated using
//!   the [`with_seed`](Rng::with_seed) method.
//! - `js`: Assumes that WebAssembly targets are being run in a JavaScript environment. See the
//!   [WebAssembly Notes](#webassembly-notes) section for more information.
//!
//! # WebAssembly Notes
//!
//! For non-WASI WASM targets, there is additional sublety to consider when utilizing the global RNG.
//! By default, `std` targets will use entropy sources in the standard library to seed the global RNG.
//! However, these sources are not available by default on WASM targets outside of WASI.
//!
//! If the `js` feature is enabled, this crate will assume that it is running in a JavaScript
//! environment. At this point, the [`getrandom`] crate will be used in order to access the available
//! entropy sources and seed the global RNG. If the `js` feature is not enabled, the global RNG will
//! use a predefined seed.
//!
//! [`fastrand-contrib`]: https://crates.io/crates/fastrand-contrib
//! [`getrandom`]: https://crates.io/crates/getrandom

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use core::convert::{TryFrom, TryInto};
use core::ops::{Bound, RangeBounds};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
mod global_rng;

#[cfg(feature = "std")]
pub use global_rng::*;

/// A random number generator.
#[derive(Debug, PartialEq, Eq)]
pub struct Rng(u64);

impl Clone for Rng {
    /// Clones the generator by creating a new generator with the same seed.
    fn clone(&self) -> Rng {
        Rng::with_seed(self.0)
    }
}

impl Rng {
    /// Generates a random `u32`.
    #[inline]
    fn gen_u32(&mut self) -> u32 {
        self.gen_u64() as u32
    }

    /// Generates a random `u64`.
    #[inline]
    fn gen_u64(&mut self) -> u64 {
        // Constants for WyRand taken from: https://github.com/wangyi-fudan/wyhash/blob/master/wyhash.h#L151
        // Updated for the final v4.2 implementation with improved constants for better entropy output.
        const WY_CONST_0: u64 = 0x2d35_8dcc_aa6c_78a5;
        const WY_CONST_1: u64 = 0x8bb8_4b93_962e_acc9;

        let s = self.0.wrapping_add(WY_CONST_0);
        self.0 = s;
        let t = u128::from(s) * u128::from(s ^ WY_CONST_1);
        (t as u64) ^ (t >> 64) as u64
    }

    /// Generates a random `u128`.
    #[inline]
    fn gen_u128(&mut self) -> u128 {
        (u128::from(self.gen_u64()) << 64) | u128::from(self.gen_u64())
    }

    /// Generates a random `u32` in `0..n`.
    #[inline]
    fn gen_mod_u32(&mut self, n: u32) -> u32 {
        // Adapted from: https://lemire.me/blog/2016/06/30/fast-random-shuffling/
        let mut r = self.gen_u32();
        let mut hi = mul_high_u32(r, n);
        let mut lo = r.wrapping_mul(n);
        if lo < n {
            let t = n.wrapping_neg() % n;
            while lo < t {
                r = self.gen_u32();
                hi = mul_high_u32(r, n);
                lo = r.wrapping_mul(n);
            }
        }
        hi
    }

    /// Generates a random `u64` in `0..n`.
    #[inline]
    fn gen_mod_u64(&mut self, n: u64) -> u64 {
        // Adapted from: https://lemire.me/blog/2016/06/30/fast-random-shuffling/
        let mut r = self.gen_u64();
        let mut hi = mul_high_u64(r, n);
        let mut lo = r.wrapping_mul(n);
        if lo < n {
            let t = n.wrapping_neg() % n;
            while lo < t {
                r = self.gen_u64();
                hi = mul_high_u64(r, n);
                lo = r.wrapping_mul(n);
            }
        }
        hi
    }

    /// Generates a random `u128` in `0..n`.
    #[inline]
    fn gen_mod_u128(&mut self, n: u128) -> u128 {
        // Adapted from: https://lemire.me/blog/2016/06/30/fast-random-shuffling/
        let mut r = self.gen_u128();
        let mut hi = mul_high_u128(r, n);
        let mut lo = r.wrapping_mul(n);
        if lo < n {
            let t = n.wrapping_neg() % n;
            while lo < t {
                r = self.gen_u128();
                hi = mul_high_u128(r, n);
                lo = r.wrapping_mul(n);
            }
        }
        hi
    }
}

/// Computes `(a * b) >> 32`.
#[inline]
fn mul_high_u32(a: u32, b: u32) -> u32 {
    (((a as u64) * (b as u64)) >> 32) as u32
}

/// Computes `(a * b) >> 64`.
#[inline]
fn mul_high_u64(a: u64, b: u64) -> u64 {
    (((a as u128) * (b as u128)) >> 64) as u64
}

/// Computes `(a * b) >> 128`.
#[inline]
fn mul_high_u128(a: u128, b: u128) -> u128 {
    // Adapted from: https://stackoverflow.com/a/28904636
    let a_lo = a as u64 as u128;
    let a_hi = (a >> 64) as u64 as u128;
    let b_lo = b as u64 as u128;
    let b_hi = (b >> 64) as u64 as u128;
    let carry = (a_lo * b_lo) >> 64;
    let carry = ((a_hi * b_lo) as u64 as u128 + (a_lo * b_hi) as u64 as u128 + carry) >> 64;
    a_hi * b_hi + ((a_hi * b_lo) >> 64) + ((a_lo * b_hi) >> 64) + carry
}

macro_rules! rng_integer {
    ($t:tt, $unsigned_t:tt, $gen:tt, $mod:tt, $doc:tt) => {
        #[doc = $doc]
        ///
        /// Panics if the range is empty.
        #[inline]
        pub fn $t(&mut self, range: impl RangeBounds<$t>) -> $t {
            let panic_empty_range = || {
                panic!(
                    "empty range: {:?}..{:?}",
                    range.start_bound(),
                    range.end_bound()
                )
            };

            let low = match range.start_bound() {
                Bound::Unbounded => core::$t::MIN,
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x.checked_add(1).unwrap_or_else(panic_empty_range),
            };

            let high = match range.end_bound() {
                Bound::Unbounded => core::$t::MAX,
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x.checked_sub(1).unwrap_or_else(panic_empty_range),
            };

            if low > high {
                panic_empty_range();
            }

            if low == core::$t::MIN && high == core::$t::MAX {
                self.$gen() as $t
            } else {
                let len = high.wrapping_sub(low).wrapping_add(1);
                low.wrapping_add(self.$mod(len as $unsigned_t as _) as $t)
            }
        }
    };
}

impl Rng {
    /// Creates a new random number generator with the initial seed.
    #[inline]
    #[must_use = "this creates a new instance of `Rng`; if you want to initialize the thread-local generator, use `fastrand::seed()` instead"]
    pub fn with_seed(seed: u64) -> Self {
        Rng(seed)
    }

    /// Clones the generator by deterministically deriving a new generator based on the initial
    /// seed.
    ///
    /// This function can be used to create a new generator that is a "spinoff" of the old
    /// generator. The new generator will not produce the same sequence of values as the
    /// old generator.
    ///
    /// # Example
    ///
    /// ```
    /// // Seed two generators equally, and clone both of them.
    /// let mut base1 = fastrand::Rng::with_seed(0x4d595df4d0f33173);
    /// base1.bool(); // Use the generator once.
    ///
    /// let mut base2 = fastrand::Rng::with_seed(0x4d595df4d0f33173);
    /// base2.bool(); // Use the generator once.
    ///
    /// let mut rng1 = base1.fork();
    /// let mut rng2 = base2.fork();
    ///
    /// println!("rng1 returns {}", rng1.u32(..));
    /// println!("rng2 returns {}", rng2.u32(..));
    /// ```
    #[inline]
    #[must_use = "this creates a new instance of `Rng`"]
    pub fn fork(&mut self) -> Self {
        Rng::with_seed(self.gen_u64())
    }

    /// Generates a random `char` in ranges a-z and A-Z.
    #[inline]
    pub fn alphabetic(&mut self) -> char {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        *self.choice(CHARS).unwrap() as char
    }

    /// Generates a random `char` in ranges a-z, A-Z and 0-9.
    #[inline]
    pub fn alphanumeric(&mut self) -> char {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        *self.choice(CHARS).unwrap() as char
    }

    /// Generates a random `bool`.
    #[inline]
    pub fn bool(&mut self) -> bool {
        self.u8(..) % 2 == 0
    }

    /// Generates a random digit in the given `base`.
    ///
    /// Digits are represented by `char`s in ranges 0-9 and a-z.
    ///
    /// Panics if the base is zero or greater than 36.
    #[inline]
    pub fn digit(&mut self, base: u32) -> char {
        if base == 0 {
            panic!("base cannot be zero");
        }
        if base > 36 {
            panic!("base cannot be larger than 36");
        }
        let num = self.u8(..base as u8);
        if num < 10 {
            (b'0' + num) as char
        } else {
            (b'a' + num - 10) as char
        }
    }

    /// Generates a random `f32` in range `0..1`.
    pub fn f32(&mut self) -> f32 {
        let b = 32;
        let f = core::f32::MANTISSA_DIGITS - 1;
        f32::from_bits((1 << (b - 2)) - (1 << f) + (self.u32(..) >> (b - f))) - 1.0
    }

    /// Generates a random `f64` in range `0..1`.
    pub fn f64(&mut self) -> f64 {
        let b = 64;
        let f = core::f64::MANTISSA_DIGITS - 1;
        f64::from_bits((1 << (b - 2)) - (1 << f) + (self.u64(..) >> (b - f))) - 1.0
    }

    /// Collects `amount` values at random from the iterable into a vector.
    ///
    /// The length of the returned vector equals `amount` unless the iterable
    /// contains insufficient elements, in which case it equals the number of
    /// elements available.
    ///
    /// Complexity is `O(n)` where `n` is the length of the iterable.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn choose_multiple<I: IntoIterator>(&mut self, source: I, amount: usize) -> Vec<I::Item> {
        // Adapted from: https://docs.rs/rand/latest/rand/seq/trait.IteratorRandom.html#method.choose_multiple
        let mut reservoir = Vec::with_capacity(amount);
        let mut iter = source.into_iter();

        reservoir.extend(iter.by_ref().take(amount));

        // Continue unless the iterator was exhausted
        //
        // note: this prevents iterators that "restart" from causing problems.
        // If the iterator stops once, then so do we.
        if reservoir.len() == amount {
            for (i, elem) in iter.enumerate() {
                let end = i + 1 + amount;
                let k = self.usize(0..end);
                if let Some(slot) = reservoir.get_mut(k) {
                    *slot = elem;
                }
            }
        } else {
            // If less than one third of the `Vec` was used, reallocate
            // so that the unused space is not wasted. There is a corner
            // case where `amount` was much less than `self.len()`.
            if reservoir.capacity() > 3 * reservoir.len() {
                reservoir.shrink_to_fit();
            }
        }
        reservoir
    }

    rng_integer!(
        i8,
        u8,
        gen_u32,
        gen_mod_u32,
        "Generates a random `i8` in the given range."
    );

    rng_integer!(
        i16,
        u16,
        gen_u32,
        gen_mod_u32,
        "Generates a random `i16` in the given range."
    );

    rng_integer!(
        i32,
        u32,
        gen_u32,
        gen_mod_u32,
        "Generates a random `i32` in the given range."
    );

    rng_integer!(
        i64,
        u64,
        gen_u64,
        gen_mod_u64,
        "Generates a random `i64` in the given range."
    );

    rng_integer!(
        i128,
        u128,
        gen_u128,
        gen_mod_u128,
        "Generates a random `i128` in the given range."
    );

    #[cfg(target_pointer_width = "16")]
    rng_integer!(
        isize,
        usize,
        gen_u32,
        gen_mod_u32,
        "Generates a random `isize` in the given range."
    );
    #[cfg(target_pointer_width = "32")]
    rng_integer!(
        isize,
        usize,
        gen_u32,
        gen_mod_u32,
        "Generates a random `isize` in the given range."
    );
    #[cfg(target_pointer_width = "64")]
    rng_integer!(
        isize,
        usize,
        gen_u64,
        gen_mod_u64,
        "Generates a random `isize` in the given range."
    );

    /// Generates a random `char` in range a-z.
    #[inline]
    pub fn lowercase(&mut self) -> char {
        const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        *self.choice(CHARS).unwrap() as char
    }

    /// Initializes this generator with the given seed.
    #[inline]
    pub fn seed(&mut self, seed: u64) {
        self.0 = seed;
    }

    /// Gives back **current** seed that is being held by this generator.
    #[inline]
    pub fn get_seed(&self) -> u64 {
        self.0
    }

    /// Choose an item from an iterator at random.
    ///
    /// This function may have an unexpected result if the `len()` property of the
    /// iterator does not match the actual number of items in the iterator. If
    /// the iterator is empty, this returns `None`.
    #[inline]
    pub fn choice<I>(&mut self, iter: I) -> Option<I::Item>
    where
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator,
    {
        let mut iter = iter.into_iter();

        // Get the item at a random index.
        let len = iter.len();
        if len == 0 {
            return None;
        }
        let index = self.usize(0..len);

        iter.nth(index)
    }

    /// Shuffles a slice randomly.
    #[inline]
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in 1..slice.len() {
            slice.swap(i, self.usize(..=i));
        }
    }

    /// Fill a byte slice with random data.
    #[inline]
    pub fn fill(&mut self, slice: &mut [u8]) {
        // We fill the slice by chunks of 8 bytes, or one block of
        // WyRand output per new state.
        let mut chunks = slice.chunks_exact_mut(core::mem::size_of::<u64>());
        for chunk in chunks.by_ref() {
            let n = self.gen_u64().to_ne_bytes();
            // Safe because the chunks are always 8 bytes exactly.
            chunk.copy_from_slice(&n);
        }

        let remainder = chunks.into_remainder();

        // Any remainder will always be less than 8 bytes.
        if !remainder.is_empty() {
            // Generate one last block of 8 bytes of entropy
            let n = self.gen_u64().to_ne_bytes();

            // Use the remaining length to copy from block
            remainder.copy_from_slice(&n[..remainder.len()]);
        }
    }

    rng_integer!(
        u8,
        u8,
        gen_u32,
        gen_mod_u32,
        "Generates a random `u8` in the given range."
    );

    rng_integer!(
        u16,
        u16,
        gen_u32,
        gen_mod_u32,
        "Generates a random `u16` in the given range."
    );

    rng_integer!(
        u32,
        u32,
        gen_u32,
        gen_mod_u32,
        "Generates a random `u32` in the given range."
    );

    rng_integer!(
        u64,
        u64,
        gen_u64,
        gen_mod_u64,
        "Generates a random `u64` in the given range."
    );

    rng_integer!(
        u128,
        u128,
        gen_u128,
        gen_mod_u128,
        "Generates a random `u128` in the given range."
    );

    #[cfg(target_pointer_width = "16")]
    rng_integer!(
        usize,
        usize,
        gen_u32,
        gen_mod_u32,
        "Generates a random `usize` in the given range."
    );
    #[cfg(target_pointer_width = "32")]
    rng_integer!(
        usize,
        usize,
        gen_u32,
        gen_mod_u32,
        "Generates a random `usize` in the given range."
    );
    #[cfg(target_pointer_width = "64")]
    rng_integer!(
        usize,
        usize,
        gen_u64,
        gen_mod_u64,
        "Generates a random `usize` in the given range."
    );

    /// Generates a random `char` in range A-Z.
    #[inline]
    pub fn uppercase(&mut self) -> char {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        *self.choice(CHARS).unwrap() as char
    }

    /// Generates a random `char` in the given range.
    ///
    /// Panics if the range is empty.
    #[inline]
    pub fn char(&mut self, range: impl RangeBounds<char>) -> char {
        let panic_empty_range = || {
            panic!(
                "empty range: {:?}..{:?}",
                range.start_bound(),
                range.end_bound()
            )
        };

        let surrogate_start = 0xd800u32;
        let surrogate_len = 0x800u32;

        let low = match range.start_bound() {
            Bound::Unbounded => 0u8 as char,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => {
                let scalar = if x as u32 == surrogate_start - 1 {
                    surrogate_start + surrogate_len
                } else {
                    x as u32 + 1
                };
                char::try_from(scalar).unwrap_or_else(|_| panic_empty_range())
            }
        };

        let high = match range.end_bound() {
            Bound::Unbounded => core::char::MAX,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => {
                let scalar = if x as u32 == surrogate_start + surrogate_len {
                    surrogate_start - 1
                } else {
                    (x as u32).wrapping_sub(1)
                };
                char::try_from(scalar).unwrap_or_else(|_| panic_empty_range())
            }
        };

        if low > high {
            panic_empty_range();
        }

        let gap = if (low as u32) < surrogate_start && (high as u32) >= surrogate_start {
            surrogate_len
        } else {
            0
        };
        let range = high as u32 - low as u32 - gap;
        let mut val = self.u32(0..=range) + low as u32;
        if val >= surrogate_start {
            val += gap;
        }
        val.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests_llm_16_3 {
    use super::*; // Adjust this import if necessary for your module structure

use crate::*;

    #[test]
    fn test_alphabetic() {
        let mut rng = Rng::default();
        let char = rng.alphabetic();
        assert!(char.is_ascii_alphabetic(), "The generated character is not alphabetic");
        assert!(char.is_lowercase() || char.is_uppercase(), "The generated character is not in the range a-z or A-Z");
    }
}

#[cfg(test)]
mod tests_llm_16_5 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_rng_bool() {
        let mut rng = Rng::new();
        let results: Vec<bool> = (0..1000).map(|_| rng.bool()).collect();
        
        let true_count = results.iter().filter(|&&value| value).count();
        let false_count = results.iter().filter(|&&value| !value).count();
        
        // We expect a roughly equal distribution of true and false
        assert!(
            true_count + false_count == 1000, 
            "Should have 1000 results, got {} true and {} false", 
            true_count, false_count
        );
        assert!((true_count > 400 && true_count < 600), "Too few true results: {}", true_count);
        assert!((false_count > 400 && false_count < 600), "Too few false results: {}", false_count);
    }
}

#[cfg(test)]
mod tests_llm_16_10 {
    use super::*;

use crate::*;
    use crate::Rng;

    #[test]
    fn test_rng_f32() {
        let mut rng = Rng::new();
        let value = rng.f32();
        assert!(value >= 0.0 && value < 1.0, "f32 value {} is out of range [0.0, 1.0)", value);
    }

    #[test]
    fn test_rng_f32_multiple() {
        let mut rng = Rng::new();
        let values: Vec<f32> = (0..1000).map(|_| rng.f32()).collect();

        for &value in &values {
            assert!(value >= 0.0 && value < 1.0, "f32 value {} is out of range [0.0, 1.0)", value);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_13 {
    use super::*;

use crate::*;
    use crate::Rng;

    #[test]
    fn test_rng_fork() {
        let mut base_rng = Rng::with_seed(0x4d595df4d0f33173);
        let mut forked_rng1 = base_rng.fork();
        let mut forked_rng2 = base_rng.fork();

        // Ensure forked instances do not produce the same sequence
        let vals1: Vec<u32> = (0..10).map(|_| forked_rng1.u32(..)).collect();
        let vals2: Vec<u32> = (0..10).map(|_| forked_rng2.u32(..)).collect();

        // Check that the two forked RNGs produce different sequences
        assert_ne!(vals1, vals2);
    }

    #[test]
    fn test_rng_fork_reproducibility() {
        let mut base_rng = Rng::with_seed(0x4d595df4d0f33173);
        let mut forked_rng1 = base_rng.fork();
        let mut forked_rng2 = base_rng.fork();

        // Ensure the same input seed produces the same forked RNG
        let vals1: Vec<u32> = (0..10).map(|_| forked_rng1.u32(..)).collect();
        let vals2: Vec<u32> = (0..10).map(|_| forked_rng2.u32(..)).collect();

        // Both should produce the same sequence
        assert_eq!(vals1, vals2);
    }
}

#[cfg(test)]
mod tests_llm_16_14 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_gen_mod_u128() {
        let mut rng = Rng::with_seed(12345);
        
        // Test with small n
        let n = 10u128;
        let result = rng.gen_mod_u128(n);
        assert!(result < n);

        // Test with a larger n
        let n = 1_000_000_000_000_000_000u128;
        let result = rng.gen_mod_u128(n);
        assert!(result < n);

        // Test edge case n = 1
        let n = 1u128;
        let result = rng.gen_mod_u128(n);
        assert_eq!(result, 0);

        // Test with maximum `u128`
        let n = u128::MAX;
        let result = rng.gen_mod_u128(n);
        assert!(result < n);
    }
}

#[cfg(test)]
mod tests_llm_16_15 {
    use super::*;

use crate::*;
    use crate::Rng;

    #[test]
    fn test_gen_mod_u32() {
        let mut rng = Rng::with_seed(12345);
        
        const N: u32 = 10;

        // Test that we get a number in the range 0..N
        for _ in 0..1000 {
            let result = rng.gen_mod_u32(N);
            assert!(result < N);
        }

        // Test edge cases
        assert_eq!(rng.gen_mod_u32(1), 0); // range [0..1]
        assert_eq!(rng.gen_mod_u32(0), 0); // should not panic and return 0
    }

    #[test]
    #[should_panic]
    fn test_gen_mod_u32_zero() {
        let mut rng = Rng::with_seed(12345);
        rng.gen_mod_u32(0); // This should panic
    }

    #[test]
    fn test_gen_mod_u32_large() {
        let mut rng = Rng::with_seed(54321);
        
        const N: u32 = 1_000_000;

        // Test that we get a number in the range 0..N
        for _ in 0..1000 {
            let result = rng.gen_mod_u32(N);
            assert!(result < N);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_16 {
    use super::*;

use crate::*;
    use crate::Rng;

    #[test]
    fn test_gen_mod_u64() {
        let mut rng = Rng::with_seed(42);
        let n = 100;

        // Test that we get a result within the expected range
        for _ in 0..1000 {
            let result = rng.gen_mod_u64(n);
            assert!(result < n, "Result {} is not less than n {}", result, n);
        }

        // Test for cases n = 1, should always return 0
        let result_zero = rng.gen_mod_u64(1);
        assert_eq!(result_zero, 0, "Result for n=1 should always be 0");

        // Test for cases n = 0 should panic (or however the function is allowed to handle that)
        // Uncomment the following line to test for panic
        // let result_zero = rng.gen_mod_u64(0); // Expect panic.
    }
}

#[cfg(test)]
mod tests_llm_16_17 {
    use super::*;

use crate::*;
    use crate::Rng;

    #[test]
    fn test_gen_u128() {
        let mut rng = Rng::with_seed(12345);
        let value = rng.gen_u128();
        
        // Ensure the value is within the range of u128
        assert!(value <= u128::MAX);
        assert!(value >= 0);

        // Testing the randomness by generating multiple values
        let mut values = std::collections::HashSet::new();
        for _ in 0..1000 {
            values.insert(rng.gen_u128());
        }
        
        // Ensure that we got more than 1 unique value
        assert!(values.len() > 1);
    }
}

#[cfg(test)]
mod tests_llm_16_18 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_gen_u32() {
        let mut rng = Rng::with_seed(0x4d595df4d0f33173);
        let value = rng.gen_u32();
        assert!(value <= u32::MAX);
    }

    #[test]
    fn test_gen_u32_multiple() {
        let mut rng = Rng::with_seed(0x4d595df4d0f33173);
        let value1 = rng.gen_u32();
        let value2 = rng.gen_u32();
        assert!(value1 <= u32::MAX);
        assert!(value2 <= u32::MAX);
        assert!(value1 != value2); // Assuming randomness
    }

    #[test]
    fn test_gen_u32_consistency() {
        let mut rng1 = Rng::with_seed(0x4d595df4d0f33173);
        let mut rng2 = Rng::with_seed(0x4d595df4d0f33173);
        let value1 = rng1.gen_u32();
        let value2 = rng2.gen_u32();
        assert_eq!(value1, value2); // Same seed should yield same results
    }
}

#[cfg(test)]
mod tests_llm_16_19 {
    use crate::Rng;

    #[test]
    fn test_gen_u64() {
        let mut rng = Rng::with_seed(0x1234_5678_9abc_def0);
        let value1 = rng.gen_u64();
        let value2 = rng.gen_u64();
        
        // Ensure that the values are different for consecutive calls
        assert_ne!(value1, value2);
        
        // Check that values are within u64 range
        assert!(value1 <= u64::MAX);
        assert!(value2 <= u64::MAX);
    }

    #[test]
    fn test_gen_u64_with_different_seeds() {
        let mut rng1 = Rng::with_seed(0x1);
        let mut rng2 = Rng::with_seed(0x2);
        
        let value1 = rng1.gen_u64();
        let value2 = rng2.gen_u64();
        
        // Ensure that the values are different for different seeds
        assert_ne!(value1, value2);
    }

    #[test]
    fn test_gen_u64_reproducibility() {
        let seed = 0xdeadbeef;
        let mut rng1 = Rng::with_seed(seed);
        let mut rng2 = Rng::with_seed(seed);
        
        for _ in 0..10 {
            let value1 = rng1.gen_u64();
            let value2 = rng2.gen_u64();
            // Both generators should produce the same value for the same seed
            assert_eq!(value1, value2);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_20 {
    use crate::Rng;

    #[test]
    fn test_get_seed() {
        let rng = Rng::with_seed(42);
        assert_eq!(rng.get_seed(), 42);
        
        let rng_clone = rng.clone();
        assert_eq!(rng_clone.get_seed(), 42);
        
        let rng_default = Rng::default();
        assert_eq!(rng_default.get_seed(), rng_default.get_seed());
    }
}

#[cfg(test)]
mod tests_llm_16_22 {
    use super::*;

use crate::*;
    use std::ops::Range;

    #[test]
    fn test_rng_i16_in_range() {
        let mut rng = Rng::with_seed(42);
        let value = rng.i16(0..10);
        assert!(value >= 0 && value < 10);
    }

    #[test]
    fn test_rng_i16_in_negative_range() {
        let mut rng = Rng::with_seed(42);
        let value = rng.i16(-10..0);
        assert!(value >= -10 && value < 0);
    }

    #[test]
    #[should_panic(expected = "empty range")]
    fn test_rng_i16_empty_range() {
        let mut rng = Rng::with_seed(42);
        rng.i16(10..10);
    }

    #[test]
    fn test_rng_i16_unbounded_range() {
        let mut rng = Rng::with_seed(42);
        let value = rng.i16(..);
        assert!(value >= i16::MIN && value <= i16::MAX);
    }

    #[test]
    fn test_rng_i16_negative_unbounded_range() {
        let mut rng = Rng::with_seed(42);
        let value = rng.i16(..0);
        assert!(value <= 0);
    }

    #[test]
    fn test_rng_i16_inclusive_range() {
        let mut rng = Rng::with_seed(42);
        let value = rng.i16(0..=5);
        assert!(value >= 0 && value <= 5);
    }

    #[test]
    fn test_rng_i16_exclusive_range() {
        let mut rng = Rng::with_seed(42);
        let value = rng.i16(0..5);
        assert!(value >= 0 && value < 5);
    }
}

#[cfg(test)]
mod tests_llm_16_23 {
    use super::*;

use crate::*;
    use std::ops::RangeInclusive;

    #[test]
    fn test_rng_i32_within_range() {
        let mut rng = Rng::with_seed(42);
        let result = rng.i32(0..=10);
        assert!(result >= 0 && result <= 10);
    }

    #[test]
    fn test_rng_i32_with_negative_range() {
        let mut rng = Rng::with_seed(42);
        let result = rng.i32(-10..=0);
        assert!(result >= -10 && result <= 0);
    }

    #[test]
    #[should_panic(expected = "empty range: _.._")]
    fn test_rng_i32_empty_range() {
        let mut rng = Rng::with_seed(42);
        rng.i32(5..5);
    }

    #[test]
    fn test_rng_i32_large_range() {
        let mut rng = Rng::with_seed(42);
        let result = rng.i32(i32::MIN..=i32::MAX);
        assert!(result >= i32::MIN && result <= i32::MAX);
    }
}

#[cfg(test)]
mod tests_llm_16_25 {
    use super::*;

use crate::*;
    use std::ops::Range;

    #[test]
    fn test_rng_i8_inclusive_range() {
        let mut rng = Rng::new();
        let value = rng.i8(0..=10);
        assert!(value >= 0 && value <= 10);
    }

    #[test]
    fn test_rng_i8_exclusive_range() {
        let mut rng = Rng::new();
        let value = rng.i8(0..10);
        assert!(value >= 0 && value < 10);
    }

    #[test]
    fn test_rng_i8_unbounded_range() {
        let mut rng = Rng::new();
        let value = rng.i8(..);
        assert!(value >= i8::MIN && value <= i8::MAX);
    }

    #[test]
    #[should_panic]
    fn test_rng_i8_empty_range() {
        let mut rng = Rng::new();
        let _value = rng.i8(10..=0);
    }

    #[test]
    #[should_panic]
    fn test_rng_i8_exclusive_empty_range() {
        let mut rng = Rng::new();
        let _value = rng.i8(10..10);
    }

    #[test]
    #[should_panic]
    fn test_rng_i8_exclusive_empty_lower_bound() {
        let mut rng = Rng::new();
        let _value = rng.i8(..=i8::MIN);
    }

    #[test]
    #[should_panic]
    fn test_rng_i8_exclusive_empty_upper_bound() {
        let mut rng = Rng::new();
        let _value = rng.i8(0..=i8::MAX);
    }
}

#[cfg(test)]
mod tests_llm_16_28 {
    use crate::Rng;

    #[test]
    fn test_seed() {
        let mut rng = Rng::new();
        let initial_seed = rng.get_seed();
        let new_seed = 123456789;

        rng.seed(new_seed);
        let seeded_value = rng.get_seed();

        assert_eq!(seeded_value, new_seed);
        assert_ne!(seeded_value, initial_seed);
    }

    #[test]
    fn test_seed_reproducibility() {
        let seed = 987654321;
        let mut rng1 = Rng::with_seed(seed);
        let mut rng2 = Rng::with_seed(seed);

        let value1 = rng1.u32(..);
        let value2 = rng2.u32(..);

        assert_eq!(value1, value2);
    }
}

#[cfg(test)]
mod tests_llm_16_29 {
    use super::*;

use crate::*;
    use crate::Rng;

    #[test]
    fn test_shuffle() {
        let mut rng = Rng::with_seed(42);
        let mut slice = [1, 2, 3, 4, 5];
        let original = slice.clone();
        
        // Shuffle the slice
        rng.shuffle(&mut slice);
        
        // Check that the length is the same
        assert_eq!(slice.len(), original.len());
        
        // Check that all elements are still present
        for &item in &original {
            assert!(slice.contains(&item));
        }
        
        // Check that the shuffled slice is not equal to the original
        assert!(slice != original);
    }

    #[test]
    fn test_shuffle_empty() {
        let mut rng = Rng::with_seed(42);
        let mut slice: Vec<i32> = Vec::new();
        rng.shuffle(&mut slice);
        assert!(slice.is_empty());
    }

    #[test]
    fn test_shuffle_single_element() {
        let mut rng = Rng::with_seed(42);
        let mut slice = [42];
        rng.shuffle(&mut slice);
        assert_eq!(slice, [42]);
    }
}

#[cfg(test)]
mod tests_llm_16_30 {
    use super::*; // Assuming the tests are in the same module, otherwise use appropriate path

use crate::*;
    use core::ops::Range;

    #[test]
    fn test_u128_generation_in_range() {
        let mut rng = Rng::new();
        let low = 10;
        let high = 100;
        let value = rng.u128(low..=high);
        assert!(value >= low && value <= high);
    }

    #[test]
    #[should_panic(expected = "empty range: ..10")]
    fn test_u128_generation_empty_range_start() {
        let mut rng = Rng::new();
        rng.u128(..10);
    }

    #[test]
    #[should_panic(expected = "empty range: 10..10")]
    fn test_u128_generation_empty_range_inclusive() {
        let mut rng = Rng::new();
        rng.u128(10..10);
    }

    #[test]
    fn test_u128_generation_full_range() {
        let mut rng = Rng::new();
        let value = rng.u128(..);
        assert!(value >= 0); // Since u128 is always >= 0
    }

    #[test]
    fn test_u128_generation_specific_values() {
        let mut rng = Rng::with_seed(0xdeadbeef);
        let value1 = rng.u128(0..=10);
        let value2 = rng.u128(0..=10);
        assert_ne!(value1, value2); // Different seeds should produce different values
    }
}

#[cfg(test)]
mod tests_llm_16_31 {
    use super::*;

use crate::*;
    use std::ops::Range;

    #[test]
    fn test_u16_within_bounds() {
        let mut rng = Rng::with_seed(12345);
        let value = rng.u16(0..100);
        assert!(value >= 0 && value < 100);
    }

    #[test]
    fn test_u16_zero_to_max() {
        let mut rng = Rng::with_seed(12345);
        let value = rng.u16(..);
        assert!(value >= 0 && value <= u16::MAX);
    }

    #[test]
    #[should_panic(expected = "empty range: ..")]
    fn test_u16_empty_range() {
        let mut rng = Rng::with_seed(12345);
        let _ = rng.u16(0..0);
    }

    #[test]
    fn test_u16_inclusive_bounds() {
        let mut rng = Rng::with_seed(12345);
        let value = rng.u16(0..=100);
        assert!(value >= 0 && value <= 100);
    }

    #[test]
    fn test_u16_exclusive_bounds() {
        let mut rng = Rng::with_seed(12345);
        let value = rng.u16(1..100);
        assert!(value >= 1 && value < 100);
    }
}

#[cfg(test)]
mod tests_llm_16_35 {
    use super::*;

use crate::*;

    #[test]
    fn test_uppercase() {
        let mut rng = Rng::default();
        let mut results = Vec::new();
        for _ in 0..1000 {
            results.push(rng.uppercase());
        }
        // Ensure all results are in the range 'A' to 'Z'
        for &ch in &results {
            assert!(ch >= 'A' && ch <= 'Z', "Character {} is out of range", ch);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_37 {
    use crate::Rng;

    #[test]
    fn test_with_seed() {
        let seed: u64 = 12345;
        let rng = Rng::with_seed(seed);
        assert_eq!(rng.get_seed(), seed);
    }

    #[test]
    fn test_with_seed_clone() {
        let seed: u64 = 67890;
        let rng1 = Rng::with_seed(seed);
        let rng2 = rng1.clone();
        assert_eq!(rng1.get_seed(), rng2.get_seed());
        assert_eq!(rng1, rng2);
    }
}

#[cfg(test)]
mod tests_llm_16_72 {
    use super::*;

use crate::*;

    #[test]
    fn test_mul_high_u32() {
        assert_eq!(mul_high_u32(0xFFFFFFFF, 0xFFFFFFFF), 0xFFFFFFFF);
        assert_eq!(mul_high_u32(1, 1), 0);
        assert_eq!(mul_high_u32(0x00000001, 0x00000002), 0);
        assert_eq!(mul_high_u32(0x00000002, 0x00000002), 0);
        assert_eq!(mul_high_u32(0x00000001, 0x00000003), 0);
        assert_eq!(mul_high_u32(0x00000003, 0x00000003), 0);
        assert_eq!(mul_high_u32(0x80000000, 0x80000000), 0x40000000);
        assert_eq!(mul_high_u32(0xFFFFFFFF, 0x1), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_73 {
    use super::*;

use crate::*;

    #[test]
    fn test_mul_high_u64() {
        assert_eq!(mul_high_u64(0, 0), 0);
        assert_eq!(mul_high_u64(1, 0), 0);
        assert_eq!(mul_high_u64(0, 1), 0);
        assert_eq!(mul_high_u64(1, 1), 0);
        assert_eq!(mul_high_u64(1, 2), 0);
        assert_eq!(mul_high_u64(2, 1), 0);
        assert_eq!(mul_high_u64(0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF), 0xFFFFFFFFFFFFFFFF);
        assert_eq!(mul_high_u64(0x0000000000000001, 0x0000000000000001), 0);
        assert_eq!(mul_high_u64(0x0000000000000001, 0x0000000000000002), 0);
        assert_eq!(mul_high_u64(0x0000000000000002, 0x0000000000000002), 0);
    }
}
