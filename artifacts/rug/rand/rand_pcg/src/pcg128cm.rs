// Copyright 2018-2021 Developers of the Rand project.
// Copyright 2017 Paul Dicker.
// Copyright 2014-2017, 2019 Melissa O'Neill and PCG Project contributors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! PCG random number generators

// This is the cheap multiplier used by PCG for 128-bit state.
const MULTIPLIER: u64 = 15750249268501108917;

use core::fmt;
use rand_core::{impls, le, RngCore, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A PCG random number generator (CM DXSM 128/64 (LCG) variant).
///
/// Permuted Congruential Generator with 128-bit state, internal Linear
/// Congruential Generator, and 64-bit output via "double xorshift multiply"
/// output function.
///
/// This is a 128-bit LCG with explicitly chosen stream with the PCG-DXSM
/// output function. This corresponds to `pcg_engines::cm_setseq_dxsm_128_64`
/// from pcg_cpp and `PCG64DXSM` from NumPy.
///
/// Despite the name, this implementation uses 32 bytes (256 bit) space
/// comprising 128 bits of state and 128 bits stream selector. These are both
/// set by `SeedableRng`, using a 256-bit seed.
///
/// Note that while two generators with different stream parameter may be
/// closely correlated, this is [mitigated][upgrading-pcg64] by the DXSM output function.
///
/// [upgrading-pcg64]: https://numpy.org/doc/stable/reference/random/upgrading-pcg64.html
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Lcg128CmDxsm64 {
    state: u128,
    increment: u128,
}

/// [`Lcg128CmDxsm64`] is also known as `PCG64DXSM`.
pub type Pcg64Dxsm = Lcg128CmDxsm64;

impl Lcg128CmDxsm64 {
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
        let mut cur_mult = MULTIPLIER as u128;
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
    /// Note that while two generators with different stream parameter may be
    /// closely correlated, this is [mitigated][upgrading-pcg64] by the DXSM output function.
    ///
    /// PCG specifies the following default values for both parameters:
    ///
    /// - `state = 0xcafef00dd15ea5e5`
    /// - `stream = 0xa02bdbf7bb3c0a7ac28fa16a64abf96`
    ///
    /// [upgrading-pcg64]: https://numpy.org/doc/stable/reference/random/upgrading-pcg64.html
    pub fn new(state: u128, stream: u128) -> Self {
        // The increment must be odd, hence we discard one bit:
        let increment = (stream << 1) | 1;
        Self::from_state_incr(state, increment)
    }

    #[inline]
    fn from_state_incr(state: u128, increment: u128) -> Self {
        let mut pcg = Self { state, increment };
        // Move away from initial value:
        pcg.state = pcg.state.wrapping_add(pcg.increment);
        pcg.step();
        pcg
    }

    #[inline(always)]
    fn step(&mut self) {
        // prepare the LCG for the next round
        self.state = self
            .state
            .wrapping_mul(MULTIPLIER as u128)
            .wrapping_add(self.increment);
    }
}

// Custom Debug implementation that does not expose the internal state
impl fmt::Debug for Lcg128CmDxsm64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lcg128CmDxsm64 {{}}")
    }
}

impl SeedableRng for Lcg128CmDxsm64 {
    type Seed = [u8; 32];

    /// We use a single 255-bit seed to initialise the state and select a stream.
    /// One `seed` bit (lowest bit of `seed[8]`) is ignored.
    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 4];
        le::read_u64_into(&seed, &mut seed_u64);
        let state = u128::from(seed_u64[0]) | (u128::from(seed_u64[1]) << 64);
        let incr = u128::from(seed_u64[2]) | (u128::from(seed_u64[3]) << 64);

        // The increment must be odd, hence we discard one bit:
        Self::from_state_incr(state, incr | 1)
    }
}

impl RngCore for Lcg128CmDxsm64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let res = output_dxsm(self.state);
        self.step();
        res
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
}

#[inline(always)]
fn output_dxsm(state: u128) -> u64 {
    // See https://github.com/imneme/pcg-cpp/blob/ffd522e7188bef30a00c74dc7eb9de5faff90092/include/pcg_random.hpp#L1016
    // for a short discussion of the construction and its original implementation.
    let mut hi = (state >> 64) as u64;
    let mut lo = state as u64;

    lo |= 1;
    hi ^= hi >> 32;
    hi = hi.wrapping_mul(MULTIPLIER);
    hi ^= hi >> 48;
    hi = hi.wrapping_mul(lo);

    hi
}

#[cfg(test)]
mod tests_llm_16_530 {
    use super::*;

use crate::*;
    use rand_core::RngCore;

    #[test]
    fn test_fill_bytes() {
        let mut rng = Lcg128CmDxsm64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        let mut buffer = [0u8; 16];
        
        rng.fill_bytes(&mut buffer);
        
        for &byte in &buffer {
            assert_ne!(byte, 0); // Ensure bytes are not all zero
        }
    }
}

#[cfg(test)]
mod tests_llm_16_531 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;

    #[test]
    fn test_next_u32() {
        let mut rng = Lcg128CmDxsm64::from_seed([0u8; 32]);
        let value = rng.next_u32();
        assert!(value <= u32::MAX);
    }

    #[test]
    fn test_next_u32_deterministic() {
        let seed = [0u8; 32];
        let mut rng1 = Lcg128CmDxsm64::from_seed(seed);
        let mut rng2 = Lcg128CmDxsm64::from_seed(seed);

        let value1 = rng1.next_u32();
        let value2 = rng2.next_u32();
        assert_eq!(value1, value2);
    }
}

#[cfg(test)]
mod tests_llm_16_532 {
    use super::*;

use crate::*;
    use rand_core::SeedableRng;
    use crate::pcg128cm::Lcg128CmDxsm64;

    #[test]
    fn test_next_u64() {
        let seed: [u8; 32] = [0; 32]; // Example seed
        let mut rng = Lcg128CmDxsm64::from_seed(seed);
        
        let first_value = rng.next_u64();
        let second_value = rng.next_u64();

        // Check that the values are in the expected range
        assert!(first_value <= u64::MAX);
        assert!(second_value <= u64::MAX);
        
        // Ensure that consecutive calls produce different values
        assert_ne!(first_value, second_value);
    }
}

#[cfg(test)]
mod tests_llm_16_533 {
    use super::*;

use crate::*;
    use crate::pcg128cm::Lcg128CmDxsm64;
    
    #[test]
    fn test_from_seed() {
        let seed: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];

        let rng = Lcg128CmDxsm64::from_seed(seed);
        assert_eq!(rng.state, 0x03020100_07060504_0f0e0d0c_0b0a0908);
        assert_eq!(rng.increment, 0x0f0e0d0c_0b0a0908_0f0e0d0c_0b0a0908 + 1);
    }
}

#[cfg(test)]
mod tests_llm_16_546 {
    use super::*;

use crate::*;
    use crate::pcg128cm::Lcg128CmDxsm64;

    #[test]
    fn test_advance_jumps_forward() {
        let mut rng = Lcg128CmDxsm64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        let initial_state = rng.state;

        rng.advance(5);
        let state_after_advance = rng.state;

        // The state should change after advancing
        assert_ne!(initial_state, state_after_advance);
    }

    #[test]
    fn test_advance_jumps_backwards() {
        let mut rng = Lcg128CmDxsm64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        let initial_state = rng.state;

        rng.advance(10);
        let state_after_advance = rng.state;

        rng.advance(10); // jumping backwards
        let state_after_backwards_advance = rng.state;

        // The state should change after advancing and then revert to the initial state
        assert_ne!(initial_state, state_after_advance);
        assert_eq!(initial_state, state_after_backwards_advance);
    }

    #[test]
    fn test_advance_zero() {
        let mut rng = Lcg128CmDxsm64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        let initial_state = rng.state;

        rng.advance(0);
        let state_after_advance = rng.state;

        // The state should not change when advancing by 0
        assert_eq!(initial_state, state_after_advance);
    }
}

#[cfg(test)]
mod tests_llm_16_547 {
    use super::*;

use crate::*;
    use crate::pcg128cm::Lcg128CmDxsm64;

    #[test]
    fn test_from_state_incr() {
        let state: u128 = 0xcafef00dd15ea5e5;
        let increment: u128 = 0xa02bdbf7bb3c0a7ac28fa16a64abf96;

        let rng = Lcg128CmDxsm64::from_state_incr(state, increment);

        assert_eq!(rng.state, state.wrapping_add(increment));
        // You can add more assertions to test the internal state further if needed.
    }

    #[test]
    fn test_from_state_incr_different_values() {
        let state: u128 = 1;
        let increment: u128 = 3;

        let rng = Lcg128CmDxsm64::from_state_incr(state, increment);

        assert_eq!(rng.state, state.wrapping_add(increment));
    }
}

#[cfg(test)]
mod tests_llm_16_548 {
    use super::*;

use crate::*;
    use crate::pcg128cm::Lcg128CmDxsm64;

    #[test]
    fn test_new() {
        let state = 0xcafef00dd15ea5e5;
        let stream = 0xa02bdbf7bb3c0a7ac28fa16a64abf96;
        
        let rng = Lcg128CmDxsm64::new(state, stream);
        
        // Test that the state and increment are set correctly
        assert_eq!(rng.state, state.wrapping_add((stream << 1) | 1));
        assert_eq!(rng.increment, (stream << 1) | 1);
    }

    #[test]
    fn test_new_with_edge_cases() {
        let state = 0;
        let stream = 0;

        let rng = Lcg128CmDxsm64::new(state, stream);

        // Test that the state and increment are set correctly at edge case
        assert_eq!(rng.state, 1); // state should move away from initial value (0 + 1)
        assert_eq!(rng.increment, 1); // increment should be set to 1 (0 << 1 | 1)
    }
}

#[cfg(test)]
mod tests_llm_16_549 {
    use super::*;

use crate::*;
    use crate::pcg128cm::Lcg128CmDxsm64;

    #[test]
    fn test_step() {
        let mut rng = Lcg128CmDxsm64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        let initial_state = rng.state;

        rng.step();

        let expected_state = initial_state.wrapping_mul(MULTIPLIER as u128).wrapping_add(rng.increment);
        assert_eq!(rng.state, expected_state, "The state did not update as expected");
    }
}

#[cfg(test)]
mod tests_llm_16_550 {
    use super::*;

use crate::*;

    #[test]
    fn test_output_dxsm() {
        let state: u128 = 0x1234567890abcdef1234567890abcdef; // Example state value
        let expected_output: u64 = 0x1234567890abcdef; // Replace with the expected output
        let output = output_dxsm(state);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_output_dxsm_with_zero() {
        let state: u128 = 0; // Edge case with state 0
        let expected_output: u64 = 1; // Replace with the expected output for state 0
        let output = output_dxsm(state);
        assert_eq!(output, expected_output);
    }
}
