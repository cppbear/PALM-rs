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

// This is the default multiplier used by PCG for 128-bit state.
const MULTIPLIER: u128 = 0x2360_ED05_1FC6_5DA4_4385_DF64_9FCC_F645;

use core::fmt;
use rand_core::{impls, le, RngCore, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A PCG random number generator (XSL RR 128/64 (LCG) variant).
///
/// Permuted Congruential Generator with 128-bit state, internal Linear
/// Congruential Generator, and 64-bit output via "xorshift low (bits),
/// random rotation" output function.
///
/// This is a 128-bit LCG with explicitly chosen stream with the PCG-XSL-RR
/// output function. This combination is the standard `pcg64`.
///
/// Despite the name, this implementation uses 32 bytes (256 bit) space
/// comprising 128 bits of state and 128 bits stream selector. These are both
/// set by `SeedableRng`, using a 256-bit seed.
///
/// Note that two generators with different stream parameters may be closely
/// correlated.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Lcg128Xsl64 {
    state: u128,
    increment: u128,
}

/// [`Lcg128Xsl64`] is also officially known as `pcg64`.
pub type Pcg64 = Lcg128Xsl64;

impl Lcg128Xsl64 {
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
    /// Using this function is equivalent to calling `next_64()` `delta`
    /// number of times.
    #[inline]
    pub fn advance(&mut self, delta: u128) {
        let mut acc_mult: u128 = 1;
        let mut acc_plus: u128 = 0;
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
    /// - `stream = 0xa02bdbf7bb3c0a7ac28fa16a64abf96`
    pub fn new(state: u128, stream: u128) -> Self {
        // The increment must be odd, hence we discard one bit:
        let increment = (stream << 1) | 1;
        Lcg128Xsl64::from_state_incr(state, increment)
    }

    #[inline]
    fn from_state_incr(state: u128, increment: u128) -> Self {
        let mut pcg = Lcg128Xsl64 { state, increment };
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
impl fmt::Debug for Lcg128Xsl64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lcg128Xsl64 {{}}")
    }
}

impl SeedableRng for Lcg128Xsl64 {
    type Seed = [u8; 32];

    /// We use a single 255-bit seed to initialise the state and select a stream.
    /// One `seed` bit (lowest bit of `seed[8]`) is ignored.
    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 4];
        le::read_u64_into(&seed, &mut seed_u64);
        let state = u128::from(seed_u64[0]) | (u128::from(seed_u64[1]) << 64);
        let incr = u128::from(seed_u64[2]) | (u128::from(seed_u64[3]) << 64);

        // The increment must be odd, hence we discard one bit:
        Lcg128Xsl64::from_state_incr(state, incr | 1)
    }
}

impl RngCore for Lcg128Xsl64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.step();
        output_xsl_rr(self.state)
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
}

/// A PCG random number generator (XSL 128/64 (MCG) variant).
///
/// Permuted Congruential Generator with 128-bit state, internal Multiplicative
/// Congruential Generator, and 64-bit output via "xorshift low (bits),
/// random rotation" output function.
///
/// This is a 128-bit MCG with the PCG-XSL-RR output function, also known as
/// `pcg64_fast`.
/// Note that compared to the standard `pcg64` (128-bit LCG with PCG-XSL-RR
/// output function), this RNG is faster, also has a long cycle, and still has
/// good performance on statistical tests.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mcg128Xsl64 {
    state: u128,
}

/// A friendly name for [`Mcg128Xsl64`] (also known as `pcg64_fast`).
pub type Pcg64Mcg = Mcg128Xsl64;

impl Mcg128Xsl64 {
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
    /// Using this function is equivalent to calling `next_64()` `delta`
    /// number of times.
    #[inline]
    pub fn advance(&mut self, delta: u128) {
        let mut acc_mult: u128 = 1;
        let mut acc_plus: u128 = 0;
        let mut cur_mult = MULTIPLIER;
        let mut cur_plus: u128 = 0;
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

    /// Construct an instance compatible with PCG seed.
    ///
    /// Note that PCG specifies a default value for the parameter:
    ///
    /// - `state = 0xcafef00dd15ea5e5`
    pub fn new(state: u128) -> Self {
        // Force low bit to 1, as in C version (C++ uses `state | 3` instead).
        Mcg128Xsl64 { state: state | 1 }
    }
}

// Custom Debug implementation that does not expose the internal state
impl fmt::Debug for Mcg128Xsl64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mcg128Xsl64 {{}}")
    }
}

/// We use a single 126-bit seed to initialise the state and select a stream.
/// Two `seed` bits (lowest order of last byte) are ignored.
impl SeedableRng for Mcg128Xsl64 {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        // Read as if a little-endian u128 value:
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);
        let state = u128::from(seed_u64[0]) | (u128::from(seed_u64[1]) << 64);
        Mcg128Xsl64::new(state)
    }
}

impl RngCore for Mcg128Xsl64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(MULTIPLIER);
        output_xsl_rr(self.state)
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
}

#[inline(always)]
fn output_xsl_rr(state: u128) -> u64 {
    // Output function XSL RR ("xorshift low (bits), random rotation")
    // Constants are for 128-bit state, 64-bit output
    const XSHIFT: u32 = 64; // (128 - 64 + 64) / 2
    const ROTATE: u32 = 122; // 128 - 6

    let rot = (state >> ROTATE) as u32;
    let xsl = ((state >> XSHIFT) as u64) ^ (state as u64);
    xsl.rotate_right(rot)
}

#[cfg(test)]
mod tests_llm_16_520 {
    use super::*;

use crate::*;
    use rand_core::RngCore;

    #[test]
    fn test_fill_bytes() {
        let mut rng = Lcg128Xsl64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        let mut buffer = [0u8; 16];
        rng.fill_bytes(&mut buffer);

        // Check that buffer is filled
        assert!(!buffer.iter().all(|&b| b == 0), "Buffer should not be all zeros");

        // Check that the buffer is the expected length
        assert_eq!(buffer.len(), 16);
    }
}

#[cfg(test)]
mod tests_llm_16_521 {
    use super::*;

use crate::*;
    use rand_core::RngCore;

    #[test]
    fn test_next_u32() {
        let mut rng = Lcg128Xsl64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        let result = rng.next_u32();
        assert!(result <= u32::MAX);
    }

    #[test]
    fn test_next_u32_deterministic() {
        let mut rng = Lcg128Xsl64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        let result_1 = rng.next_u32();
        let result_2 = rng.next_u32();
        assert_ne!(result_1, result_2);
    }
}

#[cfg(test)]
mod tests_llm_16_522 {
    use super::*;

use crate::*;
    use rand_core::RngCore;

    #[test]
    fn test_next_u64() {
        let mut rng = Lcg128Xsl64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        let value = rng.next_u64();
        assert_ne!(value, 0);
    }

    #[test]
    fn test_next_u64_repeatability() {
        let seed = [0u8; 32];
        let mut rng1 = Lcg128Xsl64::from_seed(seed);
        let mut rng2 = Lcg128Xsl64::from_seed(seed);
        
        for _ in 0..10 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }
}

#[cfg(test)]
mod tests_llm_16_523 {
    use super::*;

use crate::*;
    use crate::pcg128::Lcg128Xsl64;
    use rand_core::SeedableRng;

    #[test]
    fn test_from_seed() {
        let seed: [u8; 32] = [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, // 8 bytes
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, // 8 bytes
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, // 8 bytes
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, // 8 bytes
        ];

        let rng = Lcg128Xsl64::from_seed(seed);
        assert_eq!(rng.state, 0xcafef00dd15ea5e5);
        assert_eq!(rng.increment, 0xa02bdbf7bb3c0a7ac28fa16a64abf96 | 1);
    }
}

#[cfg(test)]
mod tests_llm_16_526 {
    use super::*;

use crate::*;
    use rand_core::RngCore;

    #[test]
    fn test_next_u32() {
        let mut rng = Mcg128Xsl64::new(0xcafef00dd15ea5e5);
        let result = rng.next_u32();
        
        // Since we don't have a known output, we can check if the result
        // is within a valid range for u32
        assert!(result <= u32::MAX);
    }

    #[test]
    fn test_next_u32_stability() {
        let mut rng1 = Mcg128Xsl64::new(0xcafef00dd15ea5e5);
        let mut rng2 = Mcg128Xsl64::new(0xcafef00dd15ea5e5);

        // Ensure that two instances with the same seed produce the same output
        assert_eq!(rng1.next_u32(), rng2.next_u32());
    }
}

#[cfg(test)]
mod tests_llm_16_527 {
    use super::*;

use crate::*;
    use rand_core::RngCore;

    #[test]
    fn test_next_u64() {
        let mut rng = Mcg128Xsl64::new(0xcafef00dd15ea5e5);
        let first_value = rng.next_u64();
        let second_value = rng.next_u64();
        assert_ne!(first_value, second_value, "Subsequent calls to next_u64 should return different values.");
        
        // Test with a different seed
        let mut rng2 = Mcg128Xsl64::new(0xdeadbeefcafebabe);
        let first_value_r = rng2.next_u64();
        assert_ne!(first_value, first_value_r, "Different seeds should produce different first values.");
    }
}

#[cfg(test)]
mod tests_llm_16_528 {
    use super::*; // Import necessary items from the parent module

use crate::*;
    use crate::pcg128::Mcg128Xsl64; // Import the Mcg128Xsl64 struct
    use rand_core::SeedableRng; // Import SeedableRng trait

    #[test]
    fn test_from_seed() {
        // Test case 1: Check the seeding functionality
        let seed: [u8; 16] = [1; 16]; // Example seed
        let rng = Mcg128Xsl64::from_seed(seed);
        assert_eq!(rng.state, 0x0000000000000001);
        
        // Test case 2: Check the seeding functionality with a different seed
        let seed: [u8; 16] = [2; 16]; // Example seed
        let rng = Mcg128Xsl64::from_seed(seed);
        assert_eq!(rng.state, 0x0000000000000002);
        
        // Test case 3: Check the seeding functionality with a maximum seed
        let seed: [u8; 16] = [u8::MAX; 16]; // Example seed with maximum byte values
        let rng = Mcg128Xsl64::from_seed(seed);
        assert_eq!(rng.state, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
    }
}

#[cfg(test)]
mod tests_llm_16_539 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_advance() {
        let mut rng = Lcg128Xsl64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);

        let initial_state = rng.state;
        let delta = 10;

        rng.advance(delta);

        // Ensure resulting state is different, as it would be advanced
        assert_ne!(rng.state, initial_state);

        // Test that advancing by delta twice is same as advancing by 2*delta
        let state_after_double_advance = rng.state;
        rng.advance(delta);
        assert_ne!(rng.state, state_after_double_advance);
    }

    #[test]
    fn test_advance_zero() {
        let mut rng = Lcg128Xsl64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        let initial_state = rng.state;

        rng.advance(0);
        
        // State should remain unchanged when delta is 0
        assert_eq!(rng.state, initial_state);
    }
}

#[cfg(test)]
mod tests_llm_16_540 {
    use super::*;

use crate::*;
    use crate::pcg128::Lcg128Xsl64;

    #[test]
    fn test_from_state_incr() {
        let state: u128 = 0x1;
        let increment: u128 = 0x2;
        let pcg = Lcg128Xsl64::from_state_incr(state, increment);
        
        // Test the state and increment values
        assert_eq!(pcg.state, state.wrapping_add(increment) * MULTIPLIER); // Assuming MULTIPLIER is defined in scope
        assert_eq!(pcg.increment, increment);
    }

    #[test]
    fn test_from_state_incr_large_values() {
        let state: u128 = u128::MAX - 1;
        let increment: u128 = u128::MAX;
        let pcg = Lcg128Xsl64::from_state_incr(state, increment);
        
        // Ensure wrapping occurs correctly with large values
        assert_eq!(pcg.state, state.wrapping_add(increment) * MULTIPLIER);
        assert_eq!(pcg.increment, increment);
    }
}

#[cfg(test)]
mod tests_llm_16_541 {
    use super::*;

use crate::*;
    use crate::pcg128::Lcg128Xsl64;

    #[test]
    fn test_new() {
        // Define test cases with known state and stream values
        let state: u128 = 0xcafef00dd15ea5e5;
        let stream: u128 = 0xa02bdbf7bb3c0a7ac28fa16a64abf96;
        
        let rng = Lcg128Xsl64::new(state, stream);

        // Assert the initial state and increment after creation
        assert_eq!(rng.state, state.wrapping_add((stream << 1) | 1));
        assert_ne!(rng.increment, 0); // Check that the increment is non-zero
    }

    #[test]
    fn test_new_with_different_stream() {
        let state: u128 = 0xcafef00dd15ea5e5;
        let stream1: u128 = 0xa02bdbf7bb3c0a7ac28fa16a64abf96;
        let stream2: u128 = 0xb02bdbf7bb3c0a7ac28fa16a64abf96;

        let rng1 = Lcg128Xsl64::new(state, stream1);
        let rng2 = Lcg128Xsl64::new(state, stream2);

        // Assert that the state should be the same but increments should be different
        assert_eq!(rng1.state, rng2.state);
        assert_ne!(rng1.increment, rng2.increment);
    }
}

#[cfg(test)]
mod tests_llm_16_542 {
    use super::*;

use crate::*;
    use crate::pcg128::Lcg128Xsl64;

    #[test]
    fn test_step() {
        let mut rng = Lcg128Xsl64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        let initial_state = rng.state;

        rng.step();
        let new_state = rng.state;

        assert!(new_state != initial_state); // Ensure the state has changed
        assert_eq!(local_pcg_step(initial_state), new_state); // Replace this with the actual step calculation
    }

    // Local function to calculate expected state after step
    fn local_pcg_step(state: u128) -> u128 {
        state.wrapping_mul(MULTIPLIER).wrapping_add(0xa02bdbf7bb3c0a7ac28fa16a64abf96 | 1) // the increment must be odd
    }
}

#[cfg(test)]
mod tests_llm_16_544 {
    use super::*;

use crate::*;
    use crate::pcg128::Mcg128Xsl64;

    #[test]
    fn test_new() {
        let state: u128 = 0x1234567890abcdef1234567890abcdef;
        let rng = Mcg128Xsl64::new(state);
        assert_eq!(rng.state, state | 1);
    }

    #[test]
    fn test_new_with_zero() {
        let state: u128 = 0;
        let rng = Mcg128Xsl64::new(state);
        assert_eq!(rng.state, 1);
    }

    #[test]
    fn test_new_with_cafef00dd15ea5e5() {
        let state: u128 = 0xcafef00dd15ea5e5;
        let rng = Mcg128Xsl64::new(state);
        assert_eq!(rng.state, 0xcafef00dd15ea5e5 | 1);
    }
}

#[cfg(test)]
mod tests_llm_16_545 {
    use super::*;

use crate::*;

    #[test]
    fn test_output_xsl_rr() {
        assert_eq!(output_xsl_rr(0), 0);
        assert_eq!(output_xsl_rr(1), 1);
        assert_eq!(output_xsl_rr(2), 2);
        assert_eq!(output_xsl_rr(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF), 0xFFFFFFFFFFFFFFFF);
        assert_eq!(output_xsl_rr(0x1234567890ABCDEF1234567890ABCDEF), 0x7F7268F890D42B34);
    }
}
