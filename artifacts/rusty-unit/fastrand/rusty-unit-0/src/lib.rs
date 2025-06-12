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

// #![no_std]
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

pub use ntest::timeout;
pub mod rusty_monitor;

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut u32_0: u32 = 9307u32;
    let mut u32_1: u32 = 729u32;
    let mut u64_0: u64 = 3063u64;
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_1: u64 = 2019u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u128_0: u128 = 6427u128;
    let mut u128_1: u128 = 6917u128;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut bool_0: bool = crate::Rng::bool(rng_4_ref_0);
    let mut f64_0: f64 = crate::global_rng::f64();
    let mut u128_2: u128 = crate::mul_high_u128(u128_1, u128_0);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut char_0: char = crate::Rng::uppercase(rng_5_ref_0);
    let mut f32_0: f32 = crate::global_rng::f32();
    let mut f64_1: f64 = crate::global_rng::f64();
    let mut bool_1: bool = crate::global_rng::bool();
    let mut u32_2: u32 = crate::Rng::gen_u32(rng_3_ref_0);
    let mut rng_6: crate::Rng = crate::Rng::fork(rng_2_ref_0);
    let mut u64_2: u64 = crate::Rng::gen_u64(rng_1_ref_0);
    crate::Rng::seed(rng_0_ref_0, u64_0);
    let mut rng_7: crate::Rng = crate::Rng::new();
    let mut u32_3: u32 = crate::mul_high_u32(u32_1, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut u64_0: u64 = 8540u64;
    let mut u64_1: u64 = 3743u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &crate::Rng = &mut rng_1;
    let mut u64_2: u64 = 8093u64;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u64_3: u64 = 7832u64;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &crate::Rng = &mut rng_3;
    let mut u64_4: u64 = 1141u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_4);
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u128_0: u128 = 1542u128;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u128_1: u128 = crate::Rng::gen_mod_u128(rng_5_ref_0, u128_0);
    let mut u64_5: u64 = crate::global_rng::get_seed();
    let mut u128_2: u128 = crate::Rng::gen_u128(rng_4_ref_0);
    let mut char_0: char = crate::global_rng::lowercase();
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut rng_7: crate::Rng = crate::Rng::new();
    let mut rng_8: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut u32_0: u32 = crate::Rng::gen_u32(rng_7_ref_0);
    let mut rng_9: crate::Rng = crate::Rng::new();
    crate::Rng::seed(rng_2_ref_0, u64_2);
    let mut u64_6: u64 = crate::Rng::get_seed(rng_1_ref_0);
    let mut f32_0: f32 = crate::global_rng::f32();
    let mut u64_7: u64 = crate::global_rng::get_seed();
    crate::Rng::seed(rng_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3() {
    rusty_monitor::set_test_id(3);
    let mut u64_0: u64 = 3878u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_1: u64 = 177u64;
    let mut u32_0: u32 = 4634u32;
    let mut u64_2: u64 = 7414u64;
    let mut u64_3: u64 = 9529u64;
    let mut u32_1: u32 = 8745u32;
    let mut u32_2: u32 = 7510u32;
    let mut u32_3: u32 = 5438u32;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut char_0: char = crate::Rng::uppercase(rng_3_ref_0);
    let mut u64_4: u64 = crate::Rng::gen_u64(rng_2_ref_0);
    let mut char_1: char = crate::Rng::digit(rng_1_ref_0, u32_3);
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut u32_4: u32 = crate::mul_high_u32(u32_2, u32_1);
    let mut u64_5: u64 = crate::mul_high_u64(u64_3, u64_2);
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u32_5: u32 = crate::Rng::gen_mod_u32(rng_4_ref_0, u32_0);
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut bool_0: bool = crate::Rng::bool(rng_5_ref_0);
    let mut char_2: char = crate::global_rng::uppercase();
    let mut char_3: char = crate::global_rng::uppercase();
    let mut u64_6: u64 = crate::Rng::gen_u64(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5() {
    rusty_monitor::set_test_id(5);
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_0: u64 = 3725u64;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut u64_1: u64 = 3119u64;
    let mut rng_3: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u128_0: u128 = 4880u128;
    let mut rng_4: crate::Rng = crate::Rng::new();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u32_0: u32 = 1434u32;
    let mut u64_2: u64 = 6677u64;
    let mut u64_3: u64 = 5186u64;
    let mut u64_4: u64 = crate::mul_high_u64(u64_3, u64_2);
    let mut char_0: char = crate::global_rng::digit(u32_0);
    let mut u128_1: u128 = crate::Rng::gen_mod_u128(rng_4_ref_0, u128_0);
    let mut u64_5: u64 = crate::global_rng::get_seed();
    let mut rng_5: crate::Rng = crate::Rng::new();
    let mut f64_0: f64 = crate::global_rng::f64();
    let mut f64_1: f64 = crate::Rng::f64(rng_3_ref_0);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    crate::Rng::seed(rng_1_ref_0, u64_0);
    let mut f64_2: f64 = crate::global_rng::f64();
    let mut f64_3: f64 = crate::global_rng::f64();
    let mut bool_0: bool = crate::global_rng::bool();
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut char_1: char = crate::global_rng::alphanumeric();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut u64_0: u64 = 2201u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u64_1: u64 = 3179u64;
    let mut rng_2: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::new();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut rng_7: crate::Rng = crate::Rng::new();
    let mut rng_7_ref_0: &crate::Rng = &mut rng_7;
    let mut u64_2: u64 = 4874u64;
    let mut rng_8: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_8_ref_0: &crate::Rng = &mut rng_8;
    let mut u64_3: u64 = 1230u64;
    let mut rng_9: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut f64_0: f64 = crate::global_rng::f64();
    let mut char_0: char = crate::global_rng::alphanumeric();
    let mut bool_0: bool = crate::Rng::eq(rng_8_ref_0, rng_7_ref_0);
    let mut u128_0: u128 = crate::Rng::gen_u128(rng_6_ref_0);
    let mut f32_0: f32 = crate::Rng::f32(rng_5_ref_0);
    let mut u64_4: u64 = crate::Rng::get_seed(rng_4_ref_0);
    let mut char_1: char = crate::Rng::lowercase(rng_3_ref_0);
    let mut char_2: char = crate::Rng::uppercase(rng_1_ref_0);
    let mut u64_5: u64 = crate::Rng::gen_u64(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8() {
    rusty_monitor::set_test_id(8);
    let mut u64_0: u64 = 8526u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_1: u64 = 6427u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u64_2: u64 = 8674u64;
    let mut rng_3: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_3: u64 = 5653u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut rng_7: crate::Rng = crate::Rng::default();
    let mut rng_7_ref_0: &crate::Rng = &mut rng_7;
    let mut tuple_0: () = crate::Rng::assert_receiver_is_total_eq(rng_7_ref_0);
    let mut f32_0: f32 = crate::Rng::f32(rng_6_ref_0);
    let mut char_0: char = crate::Rng::lowercase(rng_5_ref_0);
    let mut char_1: char = crate::Rng::lowercase(rng_4_ref_0);
    let mut u32_0: u32 = crate::Rng::gen_u32(rng_3_ref_0);
    let mut char_2: char = crate::Rng::alphabetic(rng_2_ref_0);
    let mut char_3: char = crate::Rng::lowercase(rng_1_ref_0);
    let mut char_4: char = crate::global_rng::alphanumeric();
    let mut char_5: char = crate::global_rng::alphabetic();
    let mut char_6: char = crate::global_rng::alphabetic();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10() {
    rusty_monitor::set_test_id(10);
    let mut u64_0: u64 = 1302u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u128_0: u128 = 348u128;
    let mut u128_1: u128 = 7297u128;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &crate::Rng = &mut rng_4;
    let mut u64_1: u64 = 7351u64;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut u32_0: u32 = 9651u32;
    let mut u32_1: u32 = 205u32;
    let mut bool_0: bool = crate::global_rng::bool();
    let mut f32_0: f32 = crate::global_rng::f32();
    let mut u32_2: u32 = crate::mul_high_u32(u32_1, u32_0);
    let mut char_0: char = crate::Rng::alphabetic(rng_6_ref_0);
    let mut u64_2: u64 = crate::Rng::gen_mod_u64(rng_5_ref_0, u64_1);
    let mut u64_3: u64 = crate::Rng::get_seed(rng_4_ref_0);
    let mut char_1: char = crate::Rng::alphanumeric(rng_3_ref_0);
    let mut u32_3: u32 = crate::Rng::gen_u32(rng_1_ref_0);
    let mut rng_7: crate::Rng = crate::Rng::default();
    let mut u128_2: u128 = crate::mul_high_u128(u128_1, u128_0);
    let mut u128_3: u128 = crate::Rng::gen_u128(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_11() {
    rusty_monitor::set_test_id(11);
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_0: u64 = 6613u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u64_1: u64 = 7629u64;
    let mut u64_2: u64 = 7721u64;
    let mut u128_0: u128 = 5543u128;
    let mut u128_1: u128 = 4965u128;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_3: u64 = 2150u64;
    let mut u64_4: u64 = 247u64;
    let mut rng_5: crate::Rng = crate::Rng::new();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut char_0: char = crate::global_rng::alphanumeric();
    let mut char_1: char = crate::global_rng::alphanumeric();
    let mut bool_0: bool = crate::Rng::bool(rng_5_ref_0);
    let mut u64_5: u64 = crate::mul_high_u64(u64_4, u64_3);
    let mut bool_1: bool = crate::Rng::bool(rng_4_ref_0);
    let mut u64_6: u64 = crate::Rng::get_seed(rng_3_ref_0);
    let mut tuple_0: () = crate::Rng::assert_receiver_is_total_eq(rng_2_ref_0);
    let mut u128_2: u128 = crate::mul_high_u128(u128_1, u128_0);
    let mut u64_7: u64 = crate::global_rng::get_seed();
    let mut u64_8: u64 = crate::mul_high_u64(u64_2, u64_1);
    let mut u32_0: u32 = crate::Rng::gen_u32(rng_1_ref_0);
    let mut u32_1: u32 = crate::Rng::gen_u32(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_12() {
    rusty_monitor::set_test_id(12);
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u32_0: u32 = 8808u32;
    let mut u64_0: u64 = 6985u64;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u64_1: u64 = 9181u64;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u64_2: u64 = 8134u64;
    let mut rng_3: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_3: u64 = 4756u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::new();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut u64_4: u64 = 1457u64;
    let mut u64_5: u64 = 9156u64;
    let mut rng_7: crate::Rng = crate::Rng::with_seed(u64_5);
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut bool_0: bool = crate::global_rng::bool();
    let mut u64_6: u64 = crate::Rng::gen_mod_u64(rng_7_ref_0, u64_4);
    let mut char_0: char = crate::Rng::alphanumeric(rng_6_ref_0);
    let mut char_1: char = crate::Rng::lowercase(rng_5_ref_0);
    let mut u32_1: u32 = crate::Rng::gen_u32(rng_4_ref_0);
    crate::Rng::seed(rng_2_ref_0, u64_1);
    let mut u64_7: u64 = crate::Rng::gen_mod_u64(rng_1_ref_0, u64_0);
    let mut char_2: char = crate::global_rng::digit(u32_0);
    let mut char_3: char = crate::Rng::alphabetic(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_13() {
    rusty_monitor::set_test_id(13);
    let mut u64_0: u64 = 9560u64;
    let mut u64_1: u64 = 1069u64;
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u32_0: u32 = 2861u32;
    let mut u64_2: u64 = 5111u64;
    let mut u64_3: u64 = 2403u64;
    let mut u64_4: u64 = 6573u64;
    let mut rng_2: crate::Rng = crate::Rng::with_seed(u64_4);
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u128_0: u128 = 1498u128;
    let mut u64_5: u64 = 6083u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_5);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u128_1: u128 = crate::Rng::gen_mod_u128(rng_5_ref_0, u128_0);
    let mut char_0: char = crate::global_rng::alphanumeric();
    let mut u64_6: u64 = crate::Rng::gen_u64(rng_4_ref_0);
    let mut char_1: char = crate::global_rng::alphabetic();
    let mut char_2: char = crate::Rng::uppercase(rng_3_ref_0);
    let mut u64_7: u64 = crate::mul_high_u64(u64_3, u64_2);
    let mut char_3: char = crate::global_rng::digit(u32_0);
    let mut f32_0: f32 = crate::Rng::f32(rng_1_ref_0);
    let mut u64_8: u64 = crate::Rng::gen_mod_u64(rng_0_ref_0, u64_1);
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut rng_7: crate::Rng = crate::Rng::with_seed(u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &crate::Rng = &mut rng_1;
    let mut u32_0: u32 = 5446u32;
    let mut u32_1: u32 = 552u32;
    let mut u64_0: u64 = 5768u64;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u32_2: u32 = 6002u32;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u32_3: u32 = 4000u32;
    let mut u64_1: u64 = 8667u64;
    let mut rng_6: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut char_0: char = crate::Rng::digit(rng_6_ref_0, u32_3);
    let mut char_1: char = crate::Rng::uppercase(rng_5_ref_0);
    let mut u32_4: u32 = crate::Rng::gen_mod_u32(rng_4_ref_0, u32_2);
    let mut u64_2: u64 = crate::Rng::gen_u64(rng_3_ref_0);
    let mut f32_0: f32 = crate::global_rng::f32();
    let mut char_2: char = crate::Rng::lowercase(rng_2_ref_0);
    crate::global_rng::seed(u64_0);
    let mut u32_5: u32 = crate::mul_high_u32(u32_1, u32_0);
    let mut rng_7: crate::Rng = crate::Rng::clone(rng_1_ref_0);
    let mut u32_6: u32 = crate::Rng::gen_u32(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_15() {
    rusty_monitor::set_test_id(15);
    let mut u64_0: u64 = 9120u64;
    let mut u32_0: u32 = 5629u32;
    let mut u32_1: u32 = 2939u32;
    let mut u32_2: u32 = 1305u32;
    let mut u64_1: u64 = 8327u64;
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u128_0: u128 = 2918u128;
    let mut u64_2: u64 = 1550u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut u64_3: u64 = 6471u64;
    let mut rng_3: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut u32_3: u32 = 3382u32;
    let mut rng_4: crate::Rng = crate::Rng::new();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut char_0: char = crate::Rng::digit(rng_4_ref_0, u32_3);
    let mut rng_5: crate::Rng = crate::Rng::clone(rng_2_ref_0);
    let mut u128_1: u128 = crate::Rng::gen_mod_u128(rng_1_ref_0, u128_0);
    crate::Rng::seed(rng_0_ref_0, u64_1);
    let mut rng_5_ref_0: &crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::clone(rng_5_ref_0);
    let mut char_1: char = crate::global_rng::lowercase();
    let mut f64_0: f64 = crate::global_rng::f64();
    let mut char_2: char = crate::global_rng::digit(u32_2);
    let mut char_3: char = crate::global_rng::digit(u32_1);
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut char_4: char = crate::global_rng::digit(u32_0);
    crate::global_rng::seed(u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u64_0: u64 = 5628u64;
    let mut rng_3: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_1: u64 = 9013u64;
    let mut u32_0: u32 = 4449u32;
    let mut u64_2: u64 = 1708u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut u64_3: u64 = crate::Rng::gen_u64(rng_6_ref_0);
    let mut u32_1: u32 = crate::Rng::gen_mod_u32(rng_5_ref_0, u32_0);
    crate::global_rng::seed(u64_1);
    let mut u128_0: u128 = crate::Rng::gen_u128(rng_4_ref_0);
    let mut char_0: char = crate::Rng::alphanumeric(rng_3_ref_0);
    let mut rng_7: crate::Rng = crate::Rng::new();
    let mut f64_0: f64 = crate::Rng::f64(rng_2_ref_0);
    let mut u64_4: u64 = crate::Rng::get_seed(rng_1_ref_0);
    let mut char_1: char = crate::global_rng::alphanumeric();
    let mut u64_5: u64 = crate::global_rng::get_seed();
    let mut char_2: char = crate::global_rng::lowercase();
    let mut rng_8: crate::Rng = crate::Rng::clone(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &crate::Rng = &mut rng_0;
    let mut u32_0: u32 = 31u32;
    let mut u64_0: u64 = 584u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u64_1: u64 = 3829u64;
    let mut u64_2: u64 = 2810u64;
    let mut u64_3: u64 = 9846u64;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_4: u64 = 9191u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_4);
    let mut rng_5: crate::Rng = crate::Rng::new();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut char_0: char = crate::Rng::alphanumeric(rng_5_ref_0);
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut char_1: char = crate::Rng::lowercase(rng_3_ref_0);
    let mut f64_0: f64 = crate::global_rng::f64();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u128_0: u128 = crate::Rng::gen_u128(rng_4_ref_0);
    crate::Rng::seed(rng_2_ref_0, u64_3);
    let mut u64_5: u64 = crate::mul_high_u64(u64_2, u64_1);
    let mut char_2: char = crate::Rng::alphanumeric(rng_1_ref_0);
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut rng_7: crate::Rng = crate::Rng::fork(rng_6_ref_0);
    let mut char_3: char = crate::global_rng::digit(u32_0);
    let mut tuple_0: () = crate::Rng::assert_receiver_is_total_eq(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_18() {
    rusty_monitor::set_test_id(18);
    let mut u32_0: u32 = 4522u32;
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u32_1: u32 = 7979u32;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u64_0: u64 = 5011u64;
    let mut u64_1: u64 = 2217u64;
    let mut rng_3: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_2: u64 = 1016u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u128_0: u128 = 5892u128;
    let mut u64_3: u64 = 6158u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u64_4: u64 = 101u64;
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut u64_5: u64 = crate::Rng::gen_mod_u64(rng_6_ref_0, u64_4);
    let mut u128_1: u128 = crate::Rng::gen_mod_u128(rng_5_ref_0, u128_0);
    let mut bool_0: bool = crate::Rng::bool(rng_4_ref_0);
    crate::Rng::seed(rng_3_ref_0, u64_0);
    let mut char_0: char = crate::Rng::uppercase(rng_2_ref_0);
    let mut u32_2: u32 = crate::Rng::gen_mod_u32(rng_1_ref_0, u32_1);
    let mut u64_6: u64 = crate::global_rng::get_seed();
    let mut f64_0: f64 = crate::Rng::f64(rng_0_ref_0);
    let mut char_1: char = crate::global_rng::digit(u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_20() {
    rusty_monitor::set_test_id(20);
    let mut u32_0: u32 = 9591u32;
    let mut u64_0: u64 = 8969u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_1: u64 = 8370u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &crate::Rng = &mut rng_3;
    let mut u32_1: u32 = 7214u32;
    let mut u64_2: u64 = 9475u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &crate::Rng = &mut rng_5;
    let mut u64_3: u64 = 7483u64;
    let mut rng_6: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_6_ref_0: &crate::Rng = &mut rng_6;
    let mut rng_7: crate::Rng = crate::Rng::default();
    let mut rng_7_ref_0: &crate::Rng = &mut rng_7;
    let mut u32_2: u32 = 1779u32;
    let mut char_0: char = crate::global_rng::digit(u32_2);
    let mut bool_0: bool = crate::Rng::eq(rng_7_ref_0, rng_6_ref_0);
    let mut rng_8: crate::Rng = crate::Rng::clone(rng_5_ref_0);
    let mut u32_3: u32 = crate::Rng::gen_mod_u32(rng_4_ref_0, u32_1);
    let mut tuple_0: () = crate::Rng::assert_receiver_is_total_eq(rng_3_ref_0);
    let mut bool_1: bool = crate::Rng::bool(rng_2_ref_0);
    let mut char_1: char = crate::Rng::alphabetic(rng_1_ref_0);
    let mut rng_8_ref_0: &mut crate::Rng = &mut rng_8;
    let mut bool_2: bool = crate::Rng::bool(rng_8_ref_0);
    let mut char_2: char = crate::Rng::digit(rng_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_21() {
    rusty_monitor::set_test_id(21);
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_0: u64 = 3332u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u32_0: u32 = 9184u32;
    let mut u32_1: u32 = 8050u32;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::new();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_1: u64 = 8505u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut rng_6_ref_0: &crate::Rng = &mut rng_6;
    let mut u64_2: u64 = 1696u64;
    let mut rng_7: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut u64_3: u64 = crate::Rng::gen_u64(rng_7_ref_0);
    let mut char_0: char = crate::global_rng::lowercase();
    let mut f64_0: f64 = crate::Rng::f64(rng_5_ref_0);
    let mut f64_1: f64 = crate::Rng::f64(rng_4_ref_0);
    let mut char_1: char = crate::Rng::lowercase(rng_3_ref_0);
    let mut rng_8: crate::Rng = crate::Rng::clone(rng_2_ref_0);
    let mut u32_2: u32 = crate::mul_high_u32(u32_1, u32_0);
    let mut char_2: char = crate::Rng::alphabetic(rng_1_ref_0);
    let mut u64_4: u64 = crate::Rng::gen_u64(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_22() {
    rusty_monitor::set_test_id(22);
    let mut u64_0: u64 = 7726u64;
    let mut u64_1: u64 = 8233u64;
    let mut u64_2: u64 = 5619u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_3: u64 = 3467u64;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u128_0: u128 = 5727u128;
    let mut u64_4: u64 = 3853u64;
    let mut rng_3: crate::Rng = crate::Rng::with_seed(u64_4);
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_5: u64 = 1767u64;
    let mut u32_0: u32 = 1426u32;
    let mut u32_1: u32 = 349u32;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::fork(rng_4_ref_0);
    let mut u32_2: u32 = crate::mul_high_u32(u32_1, u32_0);
    let mut rng_6: crate::Rng = crate::Rng::with_seed(u64_5);
    let mut rng_7: crate::Rng = crate::Rng::fork(rng_3_ref_0);
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut u128_1: u128 = crate::Rng::gen_mod_u128(rng_7_ref_0, u128_0);
    let mut char_0: char = crate::global_rng::lowercase();
    let mut u128_2: u128 = crate::Rng::gen_u128(rng_2_ref_0);
    crate::Rng::seed(rng_1_ref_0, u64_3);
    let mut u64_6: u64 = crate::Rng::gen_u64(rng_0_ref_0);
    let mut u64_7: u64 = crate::mul_high_u64(u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut u64_0: u64 = 1960u64;
    let mut u64_1: u64 = 292u64;
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_2: u64 = 5405u64;
    let mut u64_3: u64 = 6023u64;
    let mut u64_4: u64 = 1686u64;
    let mut u64_5: u64 = 1362u64;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u64_6: u64 = 3417u64;
    let mut rng_2: crate::Rng = crate::Rng::with_seed(u64_6);
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &crate::Rng = &mut rng_3;
    let mut u32_0: u32 = 2099u32;
    let mut u32_1: u32 = 7175u32;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut f32_0: f32 = crate::Rng::f32(rng_4_ref_0);
    let mut u32_2: u32 = crate::mul_high_u32(u32_1, u32_0);
    let mut u64_7: u64 = crate::Rng::get_seed(rng_3_ref_0);
    let mut u64_8: u64 = crate::Rng::gen_u64(rng_1_ref_0);
    let mut f64_0: f64 = crate::global_rng::f64();
    let mut u64_9: u64 = crate::mul_high_u64(u64_5, u64_4);
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    crate::Rng::seed(rng_5_ref_0, u64_2);
    let mut u64_10: u64 = crate::Rng::gen_u64(rng_0_ref_0);
    let mut u64_11: u64 = crate::mul_high_u64(u64_1, u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_26() {
    rusty_monitor::set_test_id(26);
    let mut u32_0: u32 = 8330u32;
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_0: u64 = 5311u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u64_1: u64 = 3931u64;
    let mut rng_2: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u32_1: u32 = 4155u32;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u64_2: u64 = 2917u64;
    let mut rng_6: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut rng_7: crate::Rng = crate::Rng::default();
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut u32_2: u32 = crate::Rng::gen_u32(rng_7_ref_0);
    let mut rng_8: crate::Rng = crate::Rng::fork(rng_6_ref_0);
    let mut char_0: char = crate::Rng::alphabetic(rng_5_ref_0);
    let mut char_1: char = crate::Rng::digit(rng_4_ref_0, u32_1);
    let mut char_2: char = crate::Rng::lowercase(rng_3_ref_0);
    let mut rng_8_ref_0: &mut crate::Rng = &mut rng_8;
    let mut char_3: char = crate::Rng::lowercase(rng_8_ref_0);
    let mut char_4: char = crate::Rng::alphanumeric(rng_2_ref_0);
    let mut f32_0: f32 = crate::Rng::f32(rng_1_ref_0);
    let mut u32_3: u32 = crate::Rng::gen_mod_u32(rng_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_27() {
    rusty_monitor::set_test_id(27);
    let mut u64_0: u64 = 8915u64;
    let mut u64_1: u64 = 9801u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_2: u64 = 6986u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::new();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut rng_7: crate::Rng = crate::Rng::new();
    let mut rng_7_ref_0: &crate::Rng = &mut rng_7;
    let mut tuple_0: () = crate::Rng::assert_receiver_is_total_eq(rng_7_ref_0);
    let mut char_0: char = crate::Rng::alphanumeric(rng_6_ref_0);
    let mut char_1: char = crate::global_rng::alphanumeric();
    let mut f32_0: f32 = crate::Rng::f32(rng_4_ref_0);
    let mut char_2: char = crate::global_rng::uppercase();
    let mut f32_1: f32 = crate::global_rng::f32();
    let mut f32_2: f32 = crate::Rng::f32(rng_3_ref_0);
    let mut u64_3: u64 = crate::Rng::get_seed(rng_2_ref_0);
    let mut char_3: char = crate::global_rng::alphanumeric();
    let mut char_4: char = crate::Rng::alphanumeric(rng_1_ref_0);
    let mut u64_4: u64 = crate::Rng::gen_mod_u64(rng_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_28() {
    rusty_monitor::set_test_id(28);
    let mut u64_0: u64 = 416u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &crate::Rng = &mut rng_0;
    let mut u64_1: u64 = 156u64;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u32_0: u32 = 374u32;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::new();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_2: u64 = 1612u64;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u128_0: u128 = 7608u128;
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut u32_1: u32 = 3149u32;
    let mut u64_3: u64 = 1152u64;
    let mut rng_7: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut char_0: char = crate::Rng::digit(rng_7_ref_0, u32_1);
    let mut u128_1: u128 = crate::Rng::gen_mod_u128(rng_6_ref_0, u128_0);
    crate::Rng::seed(rng_5_ref_0, u64_2);
    let mut rng_8: crate::Rng = crate::Rng::default();
    let mut f32_0: f32 = crate::Rng::f32(rng_4_ref_0);
    let mut char_1: char = crate::Rng::digit(rng_3_ref_0, u32_0);
    let mut rng_9: crate::Rng = crate::Rng::fork(rng_2_ref_0);
    let mut u64_4: u64 = crate::Rng::gen_mod_u64(rng_1_ref_0, u64_1);
    let mut char_2: char = crate::global_rng::lowercase();
    let mut tuple_0: () = crate::Rng::assert_receiver_is_total_eq(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &crate::Rng = &mut rng_0;
    let mut u64_0: u64 = 2518u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u32_0: u32 = 8492u32;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::new();
    let mut rng_5_ref_0: &crate::Rng = &mut rng_5;
    let mut u64_1: u64 = 7701u64;
    let mut rng_6: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_6_ref_0: &crate::Rng = &mut rng_6;
    let mut u32_1: u32 = 9054u32;
    let mut u64_2: u64 = 9189u64;
    let mut rng_7: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut char_0: char = crate::global_rng::alphanumeric();
    let mut u32_2: u32 = crate::Rng::gen_mod_u32(rng_7_ref_0, u32_1);
    let mut rng_8: crate::Rng = crate::Rng::clone(rng_6_ref_0);
    let mut u64_3: u64 = crate::Rng::get_seed(rng_5_ref_0);
    let mut bool_0: bool = crate::Rng::eq(rng_4_ref_0, rng_3_ref_0);
    let mut tuple_0: () = crate::Rng::assert_receiver_is_total_eq(rng_2_ref_0);
    let mut char_1: char = crate::global_rng::digit(u32_0);
    let mut rng_8_ref_0: &mut crate::Rng = &mut rng_8;
    let mut f64_0: f64 = crate::Rng::f64(rng_8_ref_0);
    let mut f64_1: f64 = crate::Rng::f64(rng_1_ref_0);
    let mut u64_4: u64 = crate::Rng::get_seed(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_30() {
    rusty_monitor::set_test_id(30);
    let mut u64_0: u64 = 8286u64;
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u128_0: u128 = 427u128;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::new();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut u128_1: u128 = 1456u128;
    let mut u128_2: u128 = 3231u128;
    let mut u128_3: u128 = crate::mul_high_u128(u128_2, u128_1);
    let mut char_0: char = crate::global_rng::alphanumeric();
    let mut char_1: char = crate::global_rng::uppercase();
    let mut f64_0: f64 = crate::Rng::f64(rng_6_ref_0);
    let mut rng_7: crate::Rng = crate::Rng::fork(rng_5_ref_0);
    let mut u32_0: u32 = crate::Rng::gen_u32(rng_4_ref_0);
    let mut char_2: char = crate::global_rng::alphabetic();
    let mut u128_4: u128 = crate::Rng::gen_u128(rng_3_ref_0);
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut rng_8: crate::Rng = crate::Rng::fork(rng_7_ref_0);
    let mut u128_5: u128 = crate::Rng::gen_mod_u128(rng_1_ref_0, u128_0);
    crate::Rng::seed(rng_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_0: u64 = 4123u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u32_0: u32 = 7562u32;
    let mut u32_1: u32 = 7773u32;
    let mut u64_1: u64 = 3664u64;
    let mut rng_2: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u64_2: u64 = 7388u64;
    let mut u32_2: u32 = 4097u32;
    let mut u32_3: u32 = 2539u32;
    let mut u32_4: u32 = 1842u32;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::new();
    let mut rng_4_ref_0: &crate::Rng = &mut rng_4;
    let mut f32_0: f32 = crate::Rng::f32(rng_3_ref_0);
    let mut u32_5: u32 = crate::mul_high_u32(u32_4, u32_3);
    let mut f32_1: f32 = crate::global_rng::f32();
    let mut char_0: char = crate::global_rng::digit(u32_2);
    let mut rng_5: crate::Rng = crate::Rng::new();
    let mut u64_3: u64 = crate::global_rng::get_seed();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    crate::global_rng::seed(u64_2);
    let mut char_1: char = crate::global_rng::uppercase();
    let mut f32_2: f32 = crate::global_rng::f32();
    let mut f32_3: f32 = crate::Rng::f32(rng_2_ref_0);
    let mut u32_6: u32 = crate::mul_high_u32(u32_1, u32_0);
    let mut char_2: char = crate::Rng::lowercase(rng_1_ref_0);
    let mut char_3: char = crate::Rng::lowercase(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut u64_0: u64 = 7054u64;
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u32_0: u32 = 1445u32;
    let mut u32_1: u32 = 2843u32;
    let mut u64_1: u64 = 585u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u128_0: u128 = 1153u128;
    let mut u128_1: u128 = 9681u128;
    let mut u64_2: u64 = 9206u64;
    let mut rng_3: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut u64_3: u64 = 6490u64;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut char_0: char = crate::Rng::alphabetic(rng_4_ref_0);
    let mut char_1: char = crate::global_rng::alphanumeric();
    let mut char_2: char = crate::global_rng::alphanumeric();
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut char_3: char = crate::global_rng::alphabetic();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u128_2: u128 = crate::mul_high_u128(u128_1, u128_0);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u128_3: u128 = crate::Rng::gen_u128(rng_5_ref_0);
    let mut bool_0: bool = crate::Rng::bool(rng_2_ref_0);
    let mut f64_0: f64 = crate::global_rng::f64();
    let mut u32_2: u32 = crate::mul_high_u32(u32_1, u32_0);
    let mut f32_0: f32 = crate::global_rng::f32();
    crate::Rng::seed(rng_0_ref_0, u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut u64_0: u64 = 9811u64;
    let mut u64_1: u64 = 5741u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_0_ref_0: &crate::Rng = &mut rng_0;
    let mut u64_2: u64 = 7829u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u128_0: u128 = 8296u128;
    let mut u64_3: u64 = 8695u64;
    let mut rng_2: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u32_0: u32 = 6401u32;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u32_1: u32 = 7881u32;
    let mut u64_4: u64 = 1444u64;
    let mut u64_5: u64 = 6321u64;
    let mut u64_6: u64 = 1423u64;
    let mut rng_4: crate::Rng = crate::Rng::new();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut char_0: char = crate::Rng::alphanumeric(rng_4_ref_0);
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_6);
    let mut u64_7: u64 = crate::mul_high_u64(u64_5, u64_4);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u32_2: u32 = crate::Rng::gen_mod_u32(rng_5_ref_0, u32_1);
    let mut u32_3: u32 = crate::Rng::gen_mod_u32(rng_3_ref_0, u32_0);
    let mut char_1: char = crate::global_rng::alphabetic();
    let mut u128_1: u128 = crate::Rng::gen_mod_u128(rng_2_ref_0, u128_0);
    let mut tuple_0: () = crate::Rng::assert_receiver_is_total_eq(rng_0_ref_0);
    let mut char_2: char = crate::global_rng::alphabetic();
    crate::global_rng::seed(u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_34() {
    rusty_monitor::set_test_id(34);
    let mut u32_0: u32 = 8653u32;
    let mut u32_1: u32 = 4156u32;
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u128_0: u128 = 1817u128;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u128_1: u128 = 1648u128;
    let mut u64_0: u64 = 5371u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u64_1: u64 = 5361u64;
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    crate::Rng::seed(rng_6_ref_0, u64_1);
    let mut u128_2: u128 = crate::Rng::gen_mod_u128(rng_5_ref_0, u128_1);
    let mut char_0: char = crate::Rng::lowercase(rng_4_ref_0);
    let mut char_1: char = crate::global_rng::alphanumeric();
    let mut char_2: char = crate::global_rng::uppercase();
    let mut u64_2: u64 = crate::Rng::gen_u64(rng_3_ref_0);
    let mut char_3: char = crate::Rng::alphanumeric(rng_2_ref_0);
    let mut u128_3: u128 = crate::Rng::gen_mod_u128(rng_1_ref_0, u128_0);
    let mut bool_0: bool = crate::global_rng::bool();
    let mut u128_4: u128 = crate::Rng::gen_u128(rng_0_ref_0);
    let mut u32_2: u32 = crate::mul_high_u32(u32_1, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut u32_0: u32 = 8450u32;
    let mut u32_1: u32 = 6880u32;
    let mut u64_0: u64 = 988u64;
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u64_1: u64 = 2086u64;
    let mut u64_2: u64 = 6054u64;
    let mut rng_3: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u128_0: u128 = 4263u128;
    let mut u64_3: u64 = 4388u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_4: u64 = 8720u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_4);
    let mut rng_5_ref_0: &crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut f64_0: f64 = crate::Rng::f64(rng_6_ref_0);
    let mut rng_7: crate::Rng = crate::Rng::clone(rng_5_ref_0);
    let mut u128_1: u128 = crate::Rng::gen_mod_u128(rng_4_ref_0, u128_0);
    let mut rng_8: crate::Rng = crate::Rng::new();
    let mut rng_8_ref_0: &crate::Rng = &mut rng_8;
    crate::Rng::seed(rng_3_ref_0, u64_1);
    let mut f32_0: f32 = crate::Rng::f32(rng_2_ref_0);
    let mut u32_2: u32 = crate::Rng::gen_u32(rng_1_ref_0);
    crate::Rng::seed(rng_0_ref_0, u64_0);
    let mut u32_3: u32 = crate::mul_high_u32(u32_1, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_37() {
    rusty_monitor::set_test_id(37);
    let mut u128_0: u128 = 4803u128;
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u128_1: u128 = 4130u128;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u128_2: u128 = 5393u128;
    let mut u128_3: u128 = 2110u128;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u64_0: u64 = 9848u64;
    let mut u64_1: u64 = 2379u64;
    let mut rng_3: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_2: u64 = 8447u64;
    let mut u64_3: u64 = 8818u64;
    let mut rng_4: crate::Rng = crate::Rng::new();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u32_0: u32 = 8012u32;
    let mut rng_5: crate::Rng = crate::Rng::new();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut char_0: char = crate::Rng::digit(rng_5_ref_0, u32_0);
    let mut u64_4: u64 = crate::Rng::gen_mod_u64(rng_4_ref_0, u64_3);
    let mut rng_7: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut u64_5: u64 = crate::Rng::gen_mod_u64(rng_3_ref_0, u64_0);
    let mut bool_0: bool = crate::Rng::bool(rng_2_ref_0);
    let mut u128_4: u128 = crate::mul_high_u128(u128_3, u128_2);
    let mut rng_8: crate::Rng = crate::Rng::new();
    let mut u128_5: u128 = crate::Rng::gen_mod_u128(rng_1_ref_0, u128_1);
    let mut char_1: char = crate::global_rng::uppercase();
    let mut u128_6: u128 = crate::Rng::gen_mod_u128(rng_0_ref_0, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut u32_0: u32 = 2662u32;
    let mut u64_0: u64 = 2192u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_1: u64 = 5950u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u32_1: u32 = 9767u32;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &crate::Rng = &mut rng_3;
    let mut u32_2: u32 = 5299u32;
    let mut rng_4: crate::Rng = crate::Rng::new();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_2: u64 = 924u64;
    let mut u64_3: u64 = 329u64;
    let mut rng_5: crate::Rng = crate::Rng::new();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut char_0: char = crate::global_rng::alphanumeric();
    let mut f64_0: f64 = crate::Rng::f64(rng_6_ref_0);
    let mut rng_7: crate::Rng = crate::Rng::new();
    let mut u64_4: u64 = crate::Rng::gen_mod_u64(rng_5_ref_0, u64_3);
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut u64_5: u64 = crate::Rng::gen_mod_u64(rng_7_ref_0, u64_2);
    let mut u32_3: u32 = crate::Rng::gen_mod_u32(rng_4_ref_0, u32_2);
    let mut rng_8: crate::Rng = crate::Rng::clone(rng_3_ref_0);
    let mut char_1: char = crate::Rng::digit(rng_2_ref_0, u32_1);
    let mut char_2: char = crate::Rng::lowercase(rng_1_ref_0);
    let mut rng_8_ref_0: &mut crate::Rng = &mut rng_8;
    let mut u64_6: u64 = crate::global_rng::get_seed();
    let mut u32_4: u32 = crate::Rng::gen_mod_u32(rng_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_40() {
    rusty_monitor::set_test_id(40);
    let mut rng_0: crate::Rng = crate::Rng::new();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_0: u64 = 6914u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u128_0: u128 = 7556u128;
    let mut u128_1: u128 = 1798u128;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u128_2: u128 = 1219u128;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_1: u64 = 6127u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_4_ref_0: &crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &crate::Rng = &mut rng_5;
    let mut u32_0: u32 = 6584u32;
    let mut u32_1: u32 = 3275u32;
    let mut u64_2: u64 = 4886u64;
    let mut rng_6: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut rng_7: crate::Rng = crate::Rng::default();
    let mut rng_7_ref_0: &crate::Rng = &mut rng_7;
    let mut u64_3: u64 = crate::Rng::get_seed(rng_7_ref_0);
    let mut u32_2: u32 = crate::Rng::gen_mod_u32(rng_6_ref_0, u32_1);
    let mut char_0: char = crate::global_rng::digit(u32_0);
    let mut u64_4: u64 = crate::Rng::get_seed(rng_5_ref_0);
    let mut u128_3: u128 = crate::Rng::gen_mod_u128(rng_3_ref_0, u128_2);
    let mut u128_4: u128 = crate::mul_high_u128(u128_1, u128_0);
    let mut char_1: char = crate::Rng::lowercase(rng_1_ref_0);
    let mut f32_0: f32 = crate::Rng::f32(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut u64_0: u64 = 9930u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u32_0: u32 = 6579u32;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u64_1: u64 = 6303u64;
    let mut u64_2: u64 = 6911u64;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_3: u64 = 6096u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_4_ref_0: &crate::Rng = &mut rng_4;
    let mut u128_0: u128 = 7239u128;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u64_4: u64 = 5071u64;
    let mut rng_6: crate::Rng = crate::Rng::with_seed(u64_4);
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut bool_0: bool = crate::Rng::bool(rng_6_ref_0);
    let mut u128_1: u128 = crate::Rng::gen_mod_u128(rng_5_ref_0, u128_0);
    let mut rng_7: crate::Rng = crate::Rng::clone(rng_4_ref_0);
    let mut rng_8: crate::Rng = crate::Rng::fork(rng_3_ref_0);
    let mut u64_5: u64 = crate::mul_high_u64(u64_2, u64_1);
    let mut char_0: char = crate::global_rng::lowercase();
    let mut char_1: char = crate::Rng::alphabetic(rng_2_ref_0);
    let mut char_2: char = crate::Rng::digit(rng_1_ref_0, u32_0);
    let mut f64_0: f64 = crate::Rng::f64(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u32_0: u32 = 3067u32;
    let mut u64_0: u64 = 3506u64;
    let mut rng_2: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::new();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u32_1: u32 = 5111u32;
    let mut u32_2: u32 = 8386u32;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut rng_7: crate::Rng = crate::Rng::default();
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut f32_0: f32 = crate::Rng::f32(rng_7_ref_0);
    let mut bool_0: bool = crate::Rng::bool(rng_6_ref_0);
    let mut char_0: char = crate::Rng::alphabetic(rng_5_ref_0);
    let mut rng_8: crate::Rng = crate::Rng::clone(rng_4_ref_0);
    let mut u32_3: u32 = crate::mul_high_u32(u32_2, u32_1);
    let mut rng_8_ref_0: &crate::Rng = &mut rng_8;
    let mut u64_1: u64 = crate::Rng::get_seed(rng_8_ref_0);
    let mut u32_4: u32 = crate::Rng::gen_u32(rng_2_ref_0);
    let mut char_1: char = crate::global_rng::digit(u32_0);
    let mut u64_2: u64 = crate::Rng::gen_u64(rng_1_ref_0);
    let mut rng_9: crate::Rng = crate::Rng::default();
    let mut u64_3: u64 = crate::Rng::get_seed(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut u64_0: u64 = 8955u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u64_1: u64 = 4523u64;
    let mut rng_1: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u64_2: u64 = 3231u64;
    let mut rng_2: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u128_0: u128 = 3933u128;
    let mut u128_1: u128 = 3647u128;
    let mut u64_3: u64 = 7664u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::new();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut char_0: char = crate::global_rng::uppercase();
    let mut u128_2: u128 = crate::Rng::gen_u128(rng_5_ref_0);
    let mut char_1: char = crate::Rng::lowercase(rng_4_ref_0);
    let mut u128_3: u128 = crate::mul_high_u128(u128_1, u128_0);
    let mut u64_4: u64 = crate::global_rng::get_seed();
    let mut char_2: char = crate::global_rng::uppercase();
    let mut char_3: char = crate::Rng::lowercase(rng_3_ref_0);
    let mut char_4: char = crate::Rng::alphanumeric(rng_2_ref_0);
    let mut char_5: char = crate::Rng::lowercase(rng_1_ref_0);
    let mut char_6: char = crate::global_rng::alphabetic();
    let mut char_7: char = crate::Rng::alphabetic(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut u64_0: u64 = 7010u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut u128_0: u128 = 4008u128;
    let mut u128_1: u128 = 3496u128;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut u64_1: u64 = 3247u64;
    let mut u64_2: u64 = 7622u64;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut u64_3: u64 = 5975u64;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut u32_0: u32 = crate::Rng::gen_u32(rng_6_ref_0);
    let mut bool_0: bool = crate::global_rng::bool();
    let mut char_0: char = crate::global_rng::lowercase();
    crate::Rng::seed(rng_3_ref_0, u64_3);
    let mut rng_7: crate::Rng = crate::Rng::clone(rng_2_ref_0);
    let mut u64_4: u64 = crate::mul_high_u64(u64_2, u64_1);
    let mut f64_0: f64 = crate::Rng::f64(rng_1_ref_0);
    let mut u128_2: u128 = crate::mul_high_u128(u128_1, u128_0);
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut char_1: char = crate::Rng::uppercase(rng_7_ref_0);
    let mut char_2: char = crate::global_rng::alphanumeric();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_46() {
    rusty_monitor::set_test_id(46);
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &crate::Rng = &mut rng_1;
    let mut u64_0: u64 = 4025u64;
    let mut u64_1: u64 = 1369u64;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &crate::Rng = &mut rng_3;
    let mut u64_2: u64 = 9757u64;
    let mut u64_3: u64 = 4997u64;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_4: u64 = 5589u64;
    let mut u64_5: u64 = 8184u64;
    let mut u64_6: u64 = 7679u64;
    let mut rng_5: crate::Rng = crate::Rng::new();
    let mut rng_5_ref_0: &crate::Rng = &mut rng_5;
    let mut u64_7: u64 = 7617u64;
    let mut rng_6: crate::Rng = crate::Rng::new();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut u64_8: u64 = crate::Rng::gen_mod_u64(rng_6_ref_0, u64_7);
    let mut tuple_0: () = crate::Rng::assert_receiver_is_total_eq(rng_5_ref_0);
    let mut u64_9: u64 = crate::mul_high_u64(u64_6, u64_5);
    crate::global_rng::seed(u64_4);
    let mut u32_0: u32 = crate::Rng::gen_u32(rng_4_ref_0);
    let mut rng_7: crate::Rng = crate::Rng::with_seed(u64_3);
    let mut rng_8: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut u64_10: u64 = crate::Rng::get_seed(rng_3_ref_0);
    let mut u64_11: u64 = crate::Rng::gen_mod_u64(rng_2_ref_0, u64_1);
    let mut rng_9: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut bool_0: bool = crate::Rng::eq(rng_1_ref_0, rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut u64_0: u64 = 9677u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_1: u64 = 2720u64;
    let mut rng_4: crate::Rng = crate::Rng::new();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_2: u64 = 5081u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut u64_3: u64 = 5588u64;
    let mut u64_4: u64 = 8307u64;
    let mut u64_5: u64 = 9773u64;
    let mut rng_6: crate::Rng = crate::Rng::with_seed(u64_5);
    let mut rng_7: crate::Rng = crate::Rng::new();
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut bool_0: bool = crate::global_rng::bool();
    let mut char_0: char = crate::global_rng::uppercase();
    let mut char_1: char = crate::global_rng::alphabetic();
    let mut f64_0: f64 = crate::Rng::f64(rng_7_ref_0);
    let mut u64_6: u64 = crate::mul_high_u64(u64_4, u64_3);
    let mut char_2: char = crate::Rng::alphabetic(rng_5_ref_0);
    let mut char_3: char = crate::global_rng::alphabetic();
    let mut u64_7: u64 = crate::Rng::gen_mod_u64(rng_4_ref_0, u64_1);
    let mut f32_0: f32 = crate::Rng::f32(rng_3_ref_0);
    let mut rng_8: crate::Rng = crate::Rng::default();
    let mut f32_1: f32 = crate::global_rng::f32();
    let mut char_4: char = crate::Rng::alphabetic(rng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut u128_0: u128 = 7838u128;
    let mut u128_1: u128 = 8479u128;
    let mut rng_0: crate::Rng = crate::Rng::default();
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::new();
    let mut rng_1_ref_0: &mut crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::default();
    let mut rng_2_ref_0: &mut crate::Rng = &mut rng_2;
    let mut u32_0: u32 = 1638u32;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut u64_0: u64 = 346u64;
    let mut rng_4: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u64_1: u64 = 5265u64;
    let mut u64_2: u64 = 3856u64;
    let mut rng_5: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    crate::Rng::seed(rng_5_ref_0, u64_1);
    let mut f64_0: f64 = crate::global_rng::f64();
    let mut u32_1: u32 = crate::Rng::gen_u32(rng_4_ref_0);
    let mut u32_2: u32 = crate::Rng::gen_mod_u32(rng_3_ref_0, u32_0);
    let mut char_0: char = crate::Rng::alphanumeric(rng_2_ref_0);
    let mut u64_3: u64 = crate::Rng::gen_u64(rng_1_ref_0);
    let mut rng_6: crate::Rng = crate::Rng::fork(rng_0_ref_0);
    let mut u128_2: u128 = crate::mul_high_u128(u128_1, u128_0);
    let mut rng_7: crate::Rng = crate::Rng::default();
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut char_1: char = crate::Rng::lowercase(rng_7_ref_0);
    let mut f32_0: f32 = crate::global_rng::f32();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_49() {
    rusty_monitor::set_test_id(49);
    let mut u64_0: u64 = 5218u64;
    let mut rng_0: crate::Rng = crate::Rng::with_seed(u64_0);
    let mut rng_0_ref_0: &mut crate::Rng = &mut rng_0;
    let mut rng_1: crate::Rng = crate::Rng::default();
    let mut rng_1_ref_0: &crate::Rng = &mut rng_1;
    let mut rng_2: crate::Rng = crate::Rng::new();
    let mut rng_2_ref_0: &crate::Rng = &mut rng_2;
    let mut rng_3: crate::Rng = crate::Rng::default();
    let mut rng_3_ref_0: &mut crate::Rng = &mut rng_3;
    let mut rng_4: crate::Rng = crate::Rng::default();
    let mut rng_4_ref_0: &mut crate::Rng = &mut rng_4;
    let mut u32_0: u32 = 849u32;
    let mut rng_5: crate::Rng = crate::Rng::default();
    let mut rng_5_ref_0: &mut crate::Rng = &mut rng_5;
    let mut rng_6: crate::Rng = crate::Rng::default();
    let mut rng_6_ref_0: &mut crate::Rng = &mut rng_6;
    let mut u64_1: u64 = 9598u64;
    let mut rng_7: crate::Rng = crate::Rng::with_seed(u64_1);
    let mut rng_7_ref_0: &mut crate::Rng = &mut rng_7;
    let mut rng_8: crate::Rng = crate::Rng::default();
    let mut rng_9: crate::Rng = crate::Rng::new();
    let mut rng_9_ref_0: &mut crate::Rng = &mut rng_9;
    let mut u64_2: u64 = 8629u64;
    let mut rng_10: crate::Rng = crate::Rng::with_seed(u64_2);
    let mut rng_10_ref_0: &mut crate::Rng = &mut rng_10;
    let mut f32_0: f32 = crate::Rng::f32(rng_10_ref_0);
    let mut u64_3: u64 = crate::Rng::gen_u64(rng_7_ref_0);
    let mut rng_11: crate::Rng = crate::Rng::fork(rng_6_ref_0);
    let mut char_0: char = crate::Rng::digit(rng_5_ref_0, u32_0);
    let mut char_1: char = crate::global_rng::uppercase();
    let mut u32_1: u32 = crate::Rng::gen_u32(rng_3_ref_0);
    let mut bool_0: bool = crate::Rng::eq(rng_2_ref_0, rng_1_ref_0);
    let mut char_2: char = crate::Rng::uppercase(rng_0_ref_0);
    panic!("From RustyUnit with love");
}
}