// Copyright 2018 Developers of the Rand project.
// Copyright 2017 Paul Dicker.
// Copyright 2014-2017 Melissa O'Neill and PCG Project contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! PCG random number generators

use core::fmt;
use rand_core::{impls, le, RngCore, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// This is the default multiplier used by PCG for 64-bit state.
const MULTIPLIER: u64 = 6364136223846793005;

/// A PCG random number generator (XSH RR 64/32 (LCG) variant).
///
/// Permuted Congruential Generator with 64-bit state, internal Linear
/// Congruential Generator, and 32-bit output via "xorshift high (bits),
/// random rotation" output function.
///
/// This is a 64-bit LCG with explicitly chosen stream with the PCG-XSH-RR
/// output function. This combination is the standard `pcg32`.
///
/// Despite the name, this implementation uses 16 bytes (128 bit) space
/// comprising 64 bits of state and 64 bits stream selector. These are both set
/// by `SeedableRng`, using a 128-bit seed.
///
/// Note that two generators with different stream parameter may be closely
/// correlated.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Lcg64Xsh32 {
    state: u64,
    increment: u64,
}

/// [`Lcg64Xsh32`] is also officially known as `pcg32`.
pub type Pcg32 = Lcg64Xsh32;

impl Lcg64Xsh32 {
    /// Multi-step advance functions (jump-ahead, jump-back)
    ///
    /// The method used here is based on Brown, "Random Number Generation
    /// with Arbitrary Stride,", Transactions of the American Nuclear
    /// Society (Nov. 1994).  The algorithm is very similar to fast
    /// exponentiation.
    ///
    /// Even though delta is an unsigned integer, we can pass a
    /// signed integer to go backwards, it just goes "the long way round".
    ///
    /// Using this function is equivalent to calling `next_32()` `delta`
    /// number of times.
    #[inline]
    pub fn advance(&mut self, delta: u64) {
        let mut acc_mult: u64 = 1;
        let mut acc_plus: u64 = 0;
        let mut cur_mult = MULTIPLIER;
        let mut cur_plus = self.increment;
        let mut mdelta = delta;

        while mdelta > 0 {
            if (mdelta & 1) != 0 {
                acc_mult = acc_mult.wrapping_mul(cur_mult);
                acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
            }
            cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
            cur_mult = cur_mult.wrapping_mul(cur_mult);
            mdelta /= 2;
        }
        self.state = acc_mult.wrapping_mul(self.state).wrapping_add(acc_plus);
    }

    /// Construct an instance compatible with PCG seed and stream.
    ///
    /// Note that the highest bit of the `stream` parameter is discarded
    /// to simplify upholding internal invariants.
    ///
    /// Note that two generators with different stream parameters may be closely
    /// correlated.
    ///
    /// PCG specifies the following default values for both parameters:
    ///
    /// - `state = 0xcafef00dd15ea5e5`
    /// - `stream = 0xa02bdbf7bb3c0a7`
    // Note: stream is 1442695040888963407u64 >> 1
    pub fn new(state: u64, stream: u64) -> Self {
        // The increment must be odd, hence we discard one bit:
        let increment = (stream << 1) | 1;
        Lcg64Xsh32::from_state_incr(state, increment)
    }

    #[inline]
    fn from_state_incr(state: u64, increment: u64) -> Self {
        let mut pcg = Lcg64Xsh32 { state, increment };
        // Move away from initial value:
        pcg.state = pcg.state.wrapping_add(pcg.increment);
        pcg.step();
        pcg
    }

    #[inline]
    fn step(&mut self) {
        // prepare the LCG for the next round
        self.state = self
            .state
            .wrapping_mul(MULTIPLIER)
            .wrapping_add(self.increment);
    }
}

// Custom Debug implementation that does not expose the internal state
impl fmt::Debug for Lcg64Xsh32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lcg64Xsh32 {{}}")
    }
}

impl SeedableRng for Lcg64Xsh32 {
    type Seed = [u8; 16];

    /// We use a single 127-bit seed to initialise the state and select a stream.
    /// One `seed` bit (lowest bit of `seed[8]`) is ignored.
    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);

        // The increment must be odd, hence we discard one bit:
        Lcg64Xsh32::from_state_incr(seed_u64[0], seed_u64[1] | 1)
    }
}

impl RngCore for Lcg64Xsh32 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let state = self.state;
        self.step();

        // Output function XSH RR: xorshift high (bits), followed by a random rotate
        // Constants are for 64-bit state, 32-bit output
        const ROTATE: u32 = 59; // 64 - 5
        const XSHIFT: u32 = 18; // (5 + 32) / 2
        const SPARE: u32 = 27; // 64 - 32 - 5

        let rot = (state >> ROTATE) as u32;
        let xsh = (((state >> XSHIFT) ^ state) >> SPARE) as u32;
        xsh.rotate_right(rot)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
}

#[cfg(test)]
mod tests_llm_16_535 {
    use super::*;

use crate::*;
    use rand_core::RngCore;

    #[test]
    fn test_fill_bytes() {
        let mut rng = Lcg64Xsh32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        let mut buffer = [0u8; 16];
        rng.fill_bytes(&mut buffer);
        
        // Assert that buffer is filled with random bytes
        // Here, we can check that the buffer is not all zeroes, indicating that
        // the RNG generated some non-deterministic output
        assert!(!buffer.iter().all(|&byte| byte == 0));
    }
}

#[cfg(test)]
mod tests_llm_16_536 {
    use super::*;

use crate::*;
    use crate::pcg64::Lcg64Xsh32;

    #[test]
    fn test_next_u32() {
        let mut rng = Lcg64Xsh32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        let val1 = rng.next_u32();
        let val2 = rng.next_u32();
        assert_ne!(val1, val2, "The two successive calls to next_u32() should return different values.");
    }

    #[test]
    fn test_next_u32_reproducibility() {
        let mut rng1 = Lcg64Xsh32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        let mut rng2 = Lcg64Xsh32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        
        for _ in 0..100 {
            assert_eq!(rng1.next_u32(), rng2.next_u32(), "The RNGs should produce the same output when initialized with the same seed.");
        }
    }
}

#[cfg(test)]
mod tests_llm_16_537 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_next_u64() {
        let mut rng = Lcg64Xsh32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        let value = rng.next_u64();
        assert_eq!(value, 0x79b8e3f78c8f5051); // Expected value for the given state and stream
    }

    #[test]
    fn test_next_u64_determinism() {
        let mut rng1 = Lcg64Xsh32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        let mut rng2 = Lcg64Xsh32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        
        for _ in 0..10 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_next_u64_multiple_calls() {
        let mut rng = Lcg64Xsh32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        
        let first = rng.next_u64();
        let second = rng.next_u64();
        
        assert_ne!(first, second); // Values should differ for different calls
    }
}

#[cfg(test)]
mod tests_llm_16_538 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_from_seed() {
        let seed: [u8; 16] = [
            0x1u8, 0x2u8, 0x3u8, 0x4u8,
            0x5u8, 0x6u8, 0x7u8, 0x8u8,
            0x9u8, 0xAu8, 0xBu8, 0xCu8,
            0xDu8, 0xEu8, 0xFu8, 0x0u8,
        ];
        
        let rng = Lcg64Xsh32::from_seed(seed);
        
        assert_eq!(rng.state, 0x0304050607080900);
        assert_eq!(rng.increment, 0x0A0B0C0D0E0F0101);
    }
}

#[cfg(test)]
mod tests_llm_16_551 {
    use super::*;

use crate::*;
    use crate::pcg64::Lcg64Xsh32;

    #[test]
    fn test_advance() {
        let mut rng = Lcg64Xsh32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        let initial_state = rng.state;

        rng.advance(1);
        let state_after_advance_1 = rng.state;

        rng.advance(5);
        let state_after_advance_5 = rng.state;

        // Check that advancing by 1 changes the state
        assert_ne!(initial_state, state_after_advance_1);
        // Check that advancing by 5 changes the state
        assert_ne!(state_after_advance_1, state_after_advance_5);
    }

    #[test]
    fn test_advance_identity() {
        let mut rng = Lcg64Xsh32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        let initial_state = rng.state;

        // Advance by 0 should not change the state
        rng.advance(0);
        assert_eq!(initial_state, rng.state);
    }

    #[test]
    fn test_advance_negative() {
        let mut rng = Lcg64Xsh32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        let initial_state = rng.state;

        // Advance by a large number, which should effectively be a backward step
        rng.advance(u64::MAX);
        assert_ne!(initial_state, rng.state);
    }
}

#[cfg(test)]
mod tests_llm_16_552 {
    use super::*;

use crate::*;
    use crate::pcg64::Lcg64Xsh32;

    #[test]
    fn test_from_state_incr() {
        let state: u64 = 0x1234567890abcdef;
        let increment: u64 = 0xabcdef0123456789;
        let rng = Lcg64Xsh32::from_state_incr(state, increment);

        // Ensure the internal state is updated correctly
        assert_eq!(rng.state, state.wrapping_add(increment) * MULTIPLIER + increment);
    }

    #[test]
    fn test_from_state_incr_increment() {
        let state: u64 = 1;
        let increment: u64 = 1;
        let rng = Lcg64Xsh32::from_state_incr(state, increment);
        assert!(rng.increment % 2 == 1); // Ensure increment is odd
    }
}

#[cfg(test)]
mod tests_llm_16_553 {
    use super::*;

use crate::*;
    use crate::pcg64::Lcg64Xsh32;

    #[test]
    fn test_new() {
        let state = 0xcafef00dd15ea5e5;
        let stream = 0xa02bdbf7bb3c0a7;

        let rng = Lcg64Xsh32::new(state, stream);
        assert_eq!(rng.state, state.wrapping_add((stream << 1) | 1));
        assert_eq!(rng.increment, (stream << 1) | 1);
    }

    #[test]
    fn test_new_with_different_parameters() {
        let state = 0x1234567890abcdef;
        let stream = 0xabcdef1234567890;

        let rng = Lcg64Xsh32::new(state, stream);
        assert_eq!(rng.state, state.wrapping_add((stream << 1) | 1));
        assert_eq!(rng.increment, (stream << 1) | 1);
    }
}

#[cfg(test)]
mod tests_llm_16_554 {
    use super::*;

use crate::*;
    use crate::pcg64::Lcg64Xsh32;

    #[test]
    fn test_step() {
        let mut rng = Lcg64Xsh32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        let initial_state = rng.state;

        // Perform the step operation
        rng.step();

        // Verify that the state has changed
        assert_ne!(rng.state, initial_state);
        
        // Store the new state for further validation
        let new_state = rng.state;

        // Perform another step operation to verify the progress
        rng.step();
        
        // Verify that the state has changed again
        assert_ne!(rng.state, new_state);
    }
}
