// Copyright 2018-2023 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{Rng, RngCore};

/// Similar to a Uniform distribution,
/// but after returning a number in the range [0,n], n is increased by 1.
pub(crate) struct IncreasingUniform<R: RngCore> {
    pub rng: R,
    n: u32,
    // Chunk is a random number in [0, (n + 1) * (n + 2) *..* (n + chunk_remaining) )
    chunk: u32,
    chunk_remaining: u8,
}

impl<R: RngCore> IncreasingUniform<R> {
    /// Create a dice roller.
    /// The next item returned will be a random number in the range [0,n]
    pub fn new(rng: R, n: u32) -> Self {
        // If n = 0, the first number returned will always be 0
        // so we don't need to generate a random number
        let chunk_remaining = if n == 0 { 1 } else { 0 };
        Self {
            rng,
            n,
            chunk: 0,
            chunk_remaining,
        }
    }

    /// Returns a number in [0,n] and increments n by 1.
    /// Generates new random bits as needed
    /// Panics if `n >= u32::MAX`
    #[inline]
    pub fn next_index(&mut self) -> usize {
        let next_n = self.n + 1;

        // There's room for further optimisation here:
        // random_range uses rejection sampling (or other method; see #1196) to avoid bias.
        // When the initial sample is biased for range 0..bound
        // it may still be viable to use for a smaller bound
        // (especially if small biases are considered acceptable).

        let next_chunk_remaining = self.chunk_remaining.checked_sub(1).unwrap_or_else(|| {
            // If the chunk is empty, generate a new chunk
            let (bound, remaining) = calculate_bound_u32(next_n);
            // bound = (n + 1) * (n + 2) *..* (n + remaining)
            self.chunk = self.rng.random_range(..bound);
            // Chunk is a random number in
            // [0, (n + 1) * (n + 2) *..* (n + remaining) )

            remaining - 1
        });

        let result = if next_chunk_remaining == 0 {
            // `chunk` is a random number in the range [0..n+1)
            // Because `chunk_remaining` is about to be set to zero
            // we do not need to clear the chunk here
            self.chunk as usize
        } else {
            // `chunk` is a random number in a range that is a multiple of n+1
            // so r will be a random number in [0..n+1)
            let r = self.chunk % next_n;
            self.chunk /= next_n;
            r as usize
        };

        self.chunk_remaining = next_chunk_remaining;
        self.n = next_n;
        result
    }
}

#[inline]
/// Calculates `bound`, `count` such that bound (m)*(m+1)*..*(m + remaining - 1)
fn calculate_bound_u32(m: u32) -> (u32, u8) {
    debug_assert!(m > 0);
    #[inline]
    const fn inner(m: u32) -> (u32, u8) {
        let mut product = m;
        let mut current = m + 1;

        loop {
            if let Some(p) = u32::checked_mul(product, current) {
                product = p;
                current += 1;
            } else {
                // Count has a maximum value of 13 for when min is 1 or 2
                let count = (current - m) as u8;
                return (product, count);
            }
        }
    }

    const RESULT2: (u32, u8) = inner(2);
    if m == 2 {
        // Making this value a constant instead of recalculating it
        // gives a significant (~50%) performance boost for small shuffles
        return RESULT2;
    }

    inner(m)
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use rand_core::RngCore;
	use std::cmp::Eq;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_11() {
    rusty_monitor::set_test_id(11);
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::Overflow;
    let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_1_ref_0: &distr::uniform::Error = &mut error_1;
    let mut error_2: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_2_ref_0: &distr::uniform::Error = &mut error_2;
    let mut u64_0: u64 = 2942u64;
    let mut u64_1: u64 = 8259u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_0;
    let mut error_3: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
    let mut error_3_ref_0: &distr::weighted::Error = &mut error_3;
    let mut error_4: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut error_4_ref_0: &distr::weighted::Error = &mut error_4;
    let mut u32_0: u32 = 350u32;
    let mut openclosed01_0: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::default();
    let mut openclosed01_0_ref_0: &crate::distr::float::OpenClosed01 = &mut openclosed01_0;
    let mut openclosed01_1: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::clone(openclosed01_0_ref_0);
    let mut tuple_0: (u32, u8) = crate::seq::increasing_uniform::calculate_bound_u32(u32_0);
    let mut bool_0: bool = crate::distr::weighted::Error::eq(error_4_ref_0, error_3_ref_0);
    let mut u64_2: u64 = crate::rngs::mock::StepRng::next_u64(steprng_0_ref_0);
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::thread_rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut threadrng_2: crate::rngs::thread::ThreadRng = crate::thread_rng();
    let mut error_5: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_5_ref_0: &distr::uniform::Error = &mut error_5;
    let mut bool_1: bool = crate::distr::uniform::Error::eq(error_5_ref_0, error_2_ref_0);
    let mut threadrng_2_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_2;
    let mut u64_3: u64 = crate::rngs::thread::ThreadRng::next_u64(threadrng_2_ref_0);
    let mut tuple_1: () = crate::distr::weighted::Error::assert_receiver_is_total_eq(error_0_ref_0);
    panic!("From RustyUnit with love");
}
}