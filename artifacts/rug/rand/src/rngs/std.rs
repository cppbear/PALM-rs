// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The standard RNG

use rand_core::{CryptoRng, RngCore, SeedableRng};

#[cfg(any(test, feature = "os_rng"))]
pub(crate) use rand_chacha::ChaCha12Core as Core;

use rand_chacha::ChaCha12Rng as Rng;

/// A strong, fast (amortized), non-portable RNG
///
/// This is the "standard" RNG, a generator with the following properties:
///
/// - Non-[portable]: any future library version may replace the algorithm
///   and results may be platform-dependent.
///   (For a portable version, use the [rand_chacha] crate directly.)
/// - [CSPRNG]: statistically good quality of randomness and [unpredictable]
/// - Fast ([amortized](https://en.wikipedia.org/wiki/Amortized_analysis)):
///   the RNG is fast for bulk generation, but the cost of method calls is not
///   consistent due to usage of an output buffer.
///
/// The current algorithm used is the ChaCha block cipher with 12 rounds. Please
/// see this relevant [rand issue] for the discussion. This may change as new
/// evidence of cipher security and performance becomes available.
///
/// ## Seeding (construction)
///
/// This generator implements the [`SeedableRng`] trait. Any method may be used,
/// but note that `seed_from_u64` is not suitable for usage where security is
/// important. Also note that, even with a fixed seed, output is not [portable].
///
/// Using a fresh seed **direct from the OS** is the most secure option:
/// ```
/// # use rand::{SeedableRng, rngs::StdRng};
/// let rng = StdRng::from_os_rng();
/// # let _: StdRng = rng;
/// ```
///
/// Seeding via [`rand::rng()`](crate::rng()) may be faster:
/// ```
/// # use rand::{SeedableRng, rngs::StdRng};
/// let rng = StdRng::from_rng(&mut rand::rng());
/// # let _: StdRng = rng;
/// ```
///
/// Any [`SeedableRng`] method may be used, but note that `seed_from_u64` is not
/// suitable where security is required. See also [Seeding RNGs] in the book.
///
/// ## Generation
///
/// The generators implements [`RngCore`] and thus also [`Rng`][crate::Rng].
/// See also the [Random Values] chapter in the book.
///
/// [portable]: https://rust-random.github.io/book/crate-reprod.html
/// [Seeding RNGs]: https://rust-random.github.io/book/guide-seeding.html
/// [unpredictable]: https://rust-random.github.io/book/guide-rngs.html#security
/// [Random Values]: https://rust-random.github.io/book/guide-values.html
/// [CSPRNG]: https://rust-random.github.io/book/guide-gen.html#cryptographically-secure-pseudo-random-number-generator
/// [rand_chacha]: https://crates.io/crates/rand_chacha
/// [rand issue]: https://github.com/rust-random/rand/issues/932
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StdRng(Rng);

impl RngCore for StdRng {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    #[inline(always)]
    fn fill_bytes(&mut self, dst: &mut [u8]) {
        self.0.fill_bytes(dst)
    }
}

impl SeedableRng for StdRng {
    // Fix to 256 bits. Changing this is a breaking change!
    type Seed = [u8; 32];

    #[inline(always)]
    fn from_seed(seed: Self::Seed) -> Self {
        StdRng(Rng::from_seed(seed))
    }
}

impl CryptoRng for StdRng {}

#[cfg(test)]
mod test {
    use crate::rngs::StdRng;
    use crate::{RngCore, SeedableRng};

    #[test]
    fn test_stdrng_construction() {
        // Test value-stability of StdRng. This is expected to break any time
        // the algorithm is changed.
        #[rustfmt::skip]
        let seed = [1,0,0,0, 23,0,0,0, 200,1,0,0, 210,30,0,0,
                    0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0];

        let target = [10719222850664546238, 14064965282130556830];

        let mut rng0 = StdRng::from_seed(seed);
        let x0 = rng0.next_u64();

        let mut rng1 = StdRng::from_rng(&mut rng0);
        let x1 = rng1.next_u64();

        assert_eq!([x0, x1], target);
    }
}

#[cfg(test)]
mod tests_llm_16_273 {
    use super::*;

use crate::*;
    use crate::rngs::StdRng;
    use crate::SeedableRng;

    #[test]
    fn test_fill_bytes() {
        // Arrange
        let mut rng = StdRng::from_seed([0u8; 32]);
        let mut buffer = [0u8; 16];

        // Act
        rng.fill_bytes(&mut buffer);

        // Assert
        // Check that the buffer is filled with non-`0` values
        assert!(buffer.iter().any(|&byte| byte != 0));
    }

    #[test]
    fn test_fill_bytes_multiple_calls() {
        // Arrange
        let mut rng = StdRng::from_seed([1u8; 32]);
        let mut buffer1 = [0u8; 16];
        let mut buffer2 = [0u8; 16];

        // Act
        rng.fill_bytes(&mut buffer1);
        rng.fill_bytes(&mut buffer2);

        // Assert
        // Check that the buffers are filled with non-`0` values
        assert!(buffer1.iter().any(|&byte| byte != 0));
        assert!(buffer2.iter().any(|&byte| byte != 0));
        
        // Check that buffers are not the same
        assert!(buffer1 != buffer2);
    }
}

#[cfg(test)]
mod tests_llm_16_274 {
    use super::*;

use crate::*;
    use crate::rngs::StdRng;
    use crate::SeedableRng;

    #[test]
    fn test_next_u32() {
        // Initialize StdRng with a known seed
        let seed: [u8; 32] = [0; 32]; // Example seed
        let mut rng = StdRng::from_seed(seed);

        // Generate a number
        let number = rng.next_u32();

        // Assert that the output is within the valid range
        assert!(number < u32::MAX);
    }

    #[test]
    fn test_next_u32_consistency() {
        // Initialize StdRng with a known seed
        let seed: [u8; 32] = [1; 32]; // Example seed
        let mut rng1 = StdRng::from_seed(seed);
        let mut rng2 = StdRng::from_seed(seed);

        // Generate a number from both RNGs
        let number1 = rng1.next_u32();
        let number2 = rng2.next_u32();

        // Assert that both RNGs produce the same output
        assert_eq!(number1, number2);
    }
}
