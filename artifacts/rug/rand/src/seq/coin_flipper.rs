// Copyright 2018-2023 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::RngCore;

pub(crate) struct CoinFlipper<R: RngCore> {
    pub rng: R,
    chunk: u32, // TODO(opt): this should depend on RNG word size
    chunk_remaining: u32,
}

impl<R: RngCore> CoinFlipper<R> {
    pub fn new(rng: R) -> Self {
        Self {
            rng,
            chunk: 0,
            chunk_remaining: 0,
        }
    }

    #[inline]
    /// Returns true with a probability of 1 / d
    /// Uses an expected two bits of randomness
    /// Panics if d == 0
    pub fn random_ratio_one_over(&mut self, d: usize) -> bool {
        debug_assert_ne!(d, 0);
        // This uses the same logic as `random_ratio` but is optimized for the case that
        // the starting numerator is one (which it always is for `Sequence::Choose()`)

        // In this case (but not `random_ratio`), this way of calculating c is always accurate
        let c = (usize::BITS - 1 - d.leading_zeros()).min(32);

        if self.flip_c_heads(c) {
            let numerator = 1 << c;
            self.random_ratio(numerator, d)
        } else {
            false
        }
    }

    #[inline]
    /// Returns true with a probability of n / d
    /// Uses an expected two bits of randomness
    fn random_ratio(&mut self, mut n: usize, d: usize) -> bool {
        // Explanation:
        // We are trying to return true with a probability of n / d
        // If n >= d, we can just return true
        // Otherwise there are two possibilities 2n < d and 2n >= d
        // In either case we flip a coin.
        // If 2n < d
        //  If it comes up tails, return false
        //  If it comes up heads, double n and start again
        //  This is fair because (0.5 * 0) + (0.5 * 2n / d) = n / d and 2n is less than d
        // (if 2n was greater than d we would effectively round it down to 1
        // by returning true)
        // If 2n >= d
        //  If it comes up tails, set n to 2n - d and start again
        //  If it comes up heads, return true
        //  This is fair because (0.5 * 1) + (0.5 * (2n - d) / d) = n / d
        //  Note that if 2n = d and the coin comes up tails, n will be set to 0
        //  before restarting which is equivalent to returning false.

        // As a performance optimization we can flip multiple coins at once
        // This is efficient because we can use the `lzcnt` intrinsic
        // We can check up to 32 flips at once but we only receive one bit of information
        // - all heads or at least one tail.

        // Let c be the number of coins to flip. 1 <= c <= 32
        // If 2n < d, n * 2^c < d
        // If the result is all heads, then set n to n * 2^c
        // If there was at least one tail, return false
        // If 2n >= d, the order of results matters so we flip one coin at a time so c = 1
        // Ideally, c will be as high as possible within these constraints

        while n < d {
            // Find a good value for c by counting leading zeros
            // This will either give the highest possible c, or 1 less than that
            let c = n
                .leading_zeros()
                .saturating_sub(d.leading_zeros() + 1)
                .clamp(1, 32);

            if self.flip_c_heads(c) {
                // All heads
                // Set n to n * 2^c
                // If 2n >= d, the while loop will exit and we will return `true`
                // If n * 2^c > `usize::MAX` we always return `true` anyway
                n = n.saturating_mul(2_usize.pow(c));
            } else {
                // At least one tail
                if c == 1 {
                    // Calculate 2n - d.
                    // We need to use wrapping as 2n might be greater than `usize::MAX`
                    let next_n = n.wrapping_add(n).wrapping_sub(d);
                    if next_n == 0 || next_n > n {
                        // This will happen if 2n < d
                        return false;
                    }
                    n = next_n;
                } else {
                    // c > 1 so 2n < d so we can return false
                    return false;
                }
            }
        }
        true
    }

    /// If the next `c` bits of randomness all represent heads, consume them, return true
    /// Otherwise return false and consume the number of heads plus one.
    /// Generates new bits of randomness when necessary (in 32 bit chunks)
    /// Has a 1 in 2 to the `c` chance of returning true
    /// `c` must be less than or equal to 32
    fn flip_c_heads(&mut self, mut c: u32) -> bool {
        debug_assert!(c <= 32);
        // Note that zeros on the left of the chunk represent heads.
        // It needs to be this way round because zeros are filled in when left shifting
        loop {
            let zeros = self.chunk.leading_zeros();

            if zeros < c {
                // The happy path - we found a 1 and can return false
                // Note that because a 1 bit was detected,
                // We cannot have run out of random bits so we don't need to check

                // First consume all of the bits read
                // Using shl seems to give worse performance for size-hinted iterators
                self.chunk = self.chunk.wrapping_shl(zeros + 1);

                self.chunk_remaining = self.chunk_remaining.saturating_sub(zeros + 1);
                return false;
            } else {
                // The number of zeros is larger than `c`
                // There are two possibilities
                if let Some(new_remaining) = self.chunk_remaining.checked_sub(c) {
                    // Those zeroes were all part of our random chunk,
                    // throw away `c` bits of randomness and return true
                    self.chunk_remaining = new_remaining;
                    self.chunk <<= c;
                    return true;
                } else {
                    // Some of those zeroes were part of the random chunk
                    // and some were part of the space behind it
                    // We need to take into account only the zeroes that were random
                    c -= self.chunk_remaining;

                    // Generate a new chunk
                    self.chunk = self.rng.next_u32();
                    self.chunk_remaining = 32;
                    // Go back to start of loop
                }
            }
        }
    }
}

#[cfg(test)]
mod tests_llm_16_432 {
    use super::*;

use crate::*;
    use rand_core::RngCore;
    use crate::rngs::mock::StepRng;

    #[test]
    fn test_flip_c_heads_all_heads() {
        let mut rng = StepRng::new(0b00000000000000000000000000000000, 1);
        let mut flipper = CoinFlipper::new(rng);
        
        assert!(flipper.flip_c_heads(32));
        assert_eq!(flipper.chunk_remaining, 32);
        assert_eq!(flipper.chunk, 0b00000000000000000000000000000000);
    }
    
    #[test]
    fn test_flip_c_heads_some_tails() {
        let mut rng = StepRng::new(0b00000000000000000000000000000001, 1);
        let mut flipper = CoinFlipper::new(rng);
        
        assert!(!flipper.flip_c_heads(32));
        assert_eq!(flipper.chunk_remaining, 31);
        assert_eq!(flipper.chunk, 0b00000000000000000000000000000010);
    }

    #[test]
    fn test_flip_c_heads_zero() {
        let mut rng = StepRng::new(0b00000000000000000000000000000000, 1);
        let mut flipper = CoinFlipper::new(rng);
        
        assert!(flipper.flip_c_heads(1));
        assert_eq!(flipper.chunk_remaining, 31);
        assert_eq!(flipper.chunk, 0b00000000000000000000000000000000);
    }

    #[test]
    fn test_flip_c_heads_insufficient_bits() {
        let mut rng = StepRng::new(0b11111111111111111111111111111111, 1);
        let mut flipper = CoinFlipper::new(rng);
        
        assert!(flipper.flip_c_heads(1));
        assert_eq!(flipper.chunk_remaining, 31);
        assert_eq!(flipper.chunk, 0b11111111111111111111111111111111);
    }

    #[test]
    #[should_panic]
    fn test_flip_c_heads_out_of_bounds() {
        let mut rng = StepRng::new(0, 1);
        let mut flipper = CoinFlipper::new(rng);
        
        flipper.flip_c_heads(33);
    }
}

#[cfg(test)]
mod tests_llm_16_433 {
    use super::*;

use crate::*;
    use crate::rngs::mock::StepRng;

    #[test]
    fn test_coin_flipper_new() {
        let initial = 42;
        let increment = 1;
        let rng = StepRng::new(initial, increment);
        let coin_flipper = CoinFlipper::new(rng);

        assert_eq!(coin_flipper.chunk, 0);
        assert_eq!(coin_flipper.chunk_remaining, 0);
    }
}

#[cfg(test)]
mod tests_llm_16_434 {
    use super::*;

use crate::*;
    use crate::rngs::mock::StepRng;

    #[test]
    fn test_random_ratio_true_case() {
        let mut rng = StepRng::new(2, 1); // Will provide predictable output
        let mut coin_flipper = CoinFlipper::new(rng);
        let result = coin_flipper.random_ratio(3, 5); // 3/5 chance of true
        assert!(result); // With a predictable RNG, assert the result based on the set input
    }

    #[test]
    fn test_random_ratio_false_case() {
        let mut rng = StepRng::new(1, 1); // Will provide predictable output
        let mut coin_flipper = CoinFlipper::new(rng);
        let result = coin_flipper.random_ratio(1, 5); // 1/5 chance of true
        assert!(!result); // With a predictable RNG, assert the result based on the set input
    }

    #[test]
    fn test_random_ratio_equal_case() {
        let mut rng = StepRng::new(4, 1); // Will provide predictable output
        let mut coin_flipper = CoinFlipper::new(rng);
        let result = coin_flipper.random_ratio(4, 4); // 1/1 chance of true
        assert!(result); // Should always return true
    }

    #[test]
    fn test_random_ratio_zero_case() {
        let mut rng = StepRng::new(2, 1); // Will provide predictable output
        let mut coin_flipper = CoinFlipper::new(rng);
        let result = coin_flipper.random_ratio(1, 1); // 1/1 chance of true
        assert!(result); // Should always return true
    }

    #[test]
    fn test_random_ratio_greater_n_case() {
        let mut rng = StepRng::new(3, 1); // Will provide predictable output
        let mut coin_flipper = CoinFlipper::new(rng);
        let result = coin_flipper.random_ratio(6, 5); // 6/5 chance of true
        assert!(result); // Should always return true
    }
}
