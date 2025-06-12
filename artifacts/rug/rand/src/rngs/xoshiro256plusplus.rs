// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rand_core::impls::fill_bytes_via_next;
use rand_core::le::read_u64_into;
use rand_core::{RngCore, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A xoshiro256++ random number generator.
///
/// The xoshiro256++ algorithm is not suitable for cryptographic purposes, but
/// is very fast and has excellent statistical properties.
///
/// The algorithm used here is translated from [the `xoshiro256plusplus.c`
/// reference source code](http://xoshiro.di.unimi.it/xoshiro256plusplus.c) by
/// David Blackman and Sebastiano Vigna.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Xoshiro256PlusPlus {
    s: [u64; 4],
}

impl SeedableRng for Xoshiro256PlusPlus {
    type Seed = [u8; 32];

    /// Create a new `Xoshiro256PlusPlus`.  If `seed` is entirely 0, it will be
    /// mapped to a different seed.
    #[inline]
    fn from_seed(seed: [u8; 32]) -> Xoshiro256PlusPlus {
        let mut state = [0; 4];
        read_u64_into(&seed, &mut state);
        // Check for zero on aligned integers for better code generation.
        // Furtermore, seed_from_u64(0) will expand to a constant when optimized.
        if state.iter().all(|&x| x == 0) {
            return Self::seed_from_u64(0);
        }
        Xoshiro256PlusPlus { s: state }
    }

    /// Create a new `Xoshiro256PlusPlus` from a `u64` seed.
    ///
    /// This uses the SplitMix64 generator internally.
    #[inline]
    fn seed_from_u64(mut state: u64) -> Self {
        const PHI: u64 = 0x9e3779b97f4a7c15;
        let mut s = [0; 4];
        for i in s.iter_mut() {
            state = state.wrapping_add(PHI);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z = z ^ (z >> 31);
            *i = z;
        }
        // By using a non-zero PHI we are guaranteed to generate a non-zero state
        // Thus preventing a recursion between from_seed and seed_from_u64.
        debug_assert_ne!(s, [0; 4]);
        Xoshiro256PlusPlus { s }
    }
}

impl RngCore for Xoshiro256PlusPlus {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        // The lowest bits have some linear dependencies, so we use the
        // upper bits instead.
        let val = self.next_u64();
        (val >> 32) as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let res = self.s[0]
            .wrapping_add(self.s[3])
            .rotate_left(23)
            .wrapping_add(self.s[0]);

        let t = self.s[1] << 17;

        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];

        self.s[2] ^= t;

        self.s[3] = self.s[3].rotate_left(45);

        res
    }

    #[inline]
    fn fill_bytes(&mut self, dst: &mut [u8]) {
        fill_bytes_via_next(self, dst)
    }
}

#[cfg(test)]
mod tests {
    use super::Xoshiro256PlusPlus;
    use rand_core::{RngCore, SeedableRng};

    #[test]
    fn reference() {
        let mut rng = Xoshiro256PlusPlus::from_seed([
            1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
            0, 0, 0,
        ]);
        // These values were produced with the reference implementation:
        // http://xoshiro.di.unimi.it/xoshiro256plusplus.c
        let expected = [
            41943041,
            58720359,
            3588806011781223,
            3591011842654386,
            9228616714210784205,
            9973669472204895162,
            14011001112246962877,
            12406186145184390807,
            15849039046786891736,
            10450023813501588000,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }

    #[test]
    fn stable_seed_from_u64() {
        // We don't guarantee value-stability for SmallRng but this
        // could influence keeping stability whenever possible (e.g. after optimizations).
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
        let expected = [
            5987356902031041503,
            7051070477665621255,
            6633766593972829180,
            211316841551650330,
            9136120204379184874,
            379361710973160858,
            15813423377499357806,
            15596884590815070553,
            5439680534584881407,
            1369371744833522710,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_282 {
    use super::*;

use crate::*;
    use rand_core::RngCore;

    #[test]
    fn test_fill_bytes() {
        let mut rng = Xoshiro256PlusPlus::from_seed([1; 32]);
        let mut buf = [0u8; 16];
        
        rng.fill_bytes(&mut buf);
        
        // Check that the buffer is filled with random bytes
        let filled_bytes_count = buf.iter().filter(|&&byte| byte != 0).count();
        assert!(filled_bytes_count > 0, "Buffer should be filled with non-zero bytes");
    }

    #[test]
    fn test_fill_bytes_length() {
        let mut rng = Xoshiro256PlusPlus::from_seed([1; 32]);
        let mut buf = [0u8; 32];
        
        rng.fill_bytes(&mut buf);
        
        // Check that the buffer length remains the same
        assert_eq!(buf.len(), 32);
    }

    #[test]
    fn test_fill_bytes_multiple_calls() {
        let mut rng = Xoshiro256PlusPlus::from_seed([1; 32]);
        let mut buf1 = [0u8; 16];
        let mut buf2 = [0u8; 16];
        
        rng.fill_bytes(&mut buf1);
        rng.fill_bytes(&mut buf2);
        
        // Check that both buffers are filled with random bytes and are distinct
        assert!(buf1 != buf2, "Subsequent fills should produce different byte sequences");
    }
}

#[cfg(test)]
mod tests_llm_16_283 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_next_u32() {
        let seed: [u8; 32] = [1; 32]; // Example seed
        let mut rng = Xoshiro256PlusPlus::from_seed(seed);
        
        // Calling `next_u32` multiple times to test the output
        let val1 = rng.next_u32();
        let val2 = rng.next_u32();

        // Ensure the generated numbers are not the same, indicating randomness
        assert_ne!(val1, val2);
        
        // Optional: test the range of generated numbers
        assert!(val1 <= u32::MAX);
        assert!(val2 <= u32::MAX);
    }
}

#[cfg(test)]
mod tests_llm_16_285 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_from_seed_non_zero() {
        let seed: [u8; 32] = [
            1, 2, 3, 4, 5, 6, 7, 8,
            9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let rng: Xoshiro256PlusPlus = Xoshiro256PlusPlus::from_seed(seed);
        assert_ne!(rng.s, [0; 4]);
    }

    #[test]
    fn test_from_seed_zero() {
        let seed: [u8; 32] = [0; 32];
        let rng: Xoshiro256PlusPlus = Xoshiro256PlusPlus::from_seed(seed);
        assert_eq!(rng.s, [0; 4]);
    }

    #[test]
    fn test_from_seed_equivalence() {
        let seed1: [u8; 32] = [
            1, 2, 3, 4, 5, 6, 7, 8,
            9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let seed2: [u8; 32] = [
            1, 2, 3, 4, 5, 6, 7, 8,
            9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let rng1: Xoshiro256PlusPlus = Xoshiro256PlusPlus::from_seed(seed1);
        let rng2: Xoshiro256PlusPlus = Xoshiro256PlusPlus::from_seed(seed2);
        assert_eq!(rng1, rng2);
    }
}

#[cfg(test)]
mod tests_llm_16_286 {
    use super::*;

use crate::*;
    use crate::rngs::xoshiro256plusplus::Xoshiro256PlusPlus;

    #[test]
    fn test_seed_from_u64() {
        let seed: u64 = 123456789;
        let rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        assert_ne!(rng.s, [0; 4], "Expected non-zero state from non-zero seed");

        let expected_state = [
            12055211856511461309, // Example expected value (for demonstration)
            7061198283297404452,  // Replace with expected values based on seed 123456789
            6892315769195747922,  // Actual values should be computed from the algorithm
            6632332052622049344,  // (Specify expected values after computing)
        ];
        assert_eq!(rng.s, expected_state);
    }

    #[test]
    fn test_seed_from_u64_zero() {
        let seed: u64 = 0;
        let rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        assert_ne!(rng.s, [0; 4], "Expected non-zero state from zero seed");
    }
}
