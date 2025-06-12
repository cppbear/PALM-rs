// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Bernoulli distribution `Bernoulli(p)`.

use crate::distr::Distribution;
use crate::Rng;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The [Bernoulli distribution](https://en.wikipedia.org/wiki/Bernoulli_distribution) `Bernoulli(p)`.
///
/// This distribution describes a single boolean random variable, which is true
/// with probability `p` and false with probability `1 - p`.
/// It is a special case of the Binomial distribution with `n = 1`.
///
/// # Plot
///
/// The following plot shows the Bernoulli distribution with `p = 0.1`,
/// `p = 0.5`, and `p = 0.9`.
///
/// ![Bernoulli distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/bernoulli.svg)
///
/// # Example
///
/// ```rust
/// use rand::distr::{Bernoulli, Distribution};
///
/// let d = Bernoulli::new(0.3).unwrap();
/// let v = d.sample(&mut rand::rng());
/// println!("{} is from a Bernoulli distribution", v);
/// ```
///
/// # Precision
///
/// This `Bernoulli` distribution uses 64 bits from the RNG (a `u64`),
/// so only probabilities that are multiples of 2<sup>-64</sup> can be
/// represented.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bernoulli {
    /// Probability of success, relative to the maximal integer.
    p_int: u64,
}

// To sample from the Bernoulli distribution we use a method that compares a
// random `u64` value `v < (p * 2^64)`.
//
// If `p == 1.0`, the integer `v` to compare against can not represented as a
// `u64`. We manually set it to `u64::MAX` instead (2^64 - 1 instead of 2^64).
// Note that  value of `p < 1.0` can never result in `u64::MAX`, because an
// `f64` only has 53 bits of precision, and the next largest value of `p` will
// result in `2^64 - 2048`.
//
// Also there is a 100% theoretical concern: if someone consistently wants to
// generate `true` using the Bernoulli distribution (i.e. by using a probability
// of `1.0`), just using `u64::MAX` is not enough. On average it would return
// false once every 2^64 iterations. Some people apparently care about this
// case.
//
// That is why we special-case `u64::MAX` to always return `true`, without using
// the RNG, and pay the performance price for all uses that *are* reasonable.
// Luckily, if `new()` and `sample` are close, the compiler can optimize out the
// extra check.
const ALWAYS_TRUE: u64 = u64::MAX;

// This is just `2.0.powi(64)`, but written this way because it is not available
// in `no_std` mode.
const SCALE: f64 = 2.0 * (1u64 << 63) as f64;

/// Error type returned from [`Bernoulli::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BernoulliError {
    /// `p < 0` or `p > 1`.
    InvalidProbability,
}

impl fmt::Display for BernoulliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            BernoulliError::InvalidProbability => "p is outside [0, 1] in Bernoulli distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BernoulliError {}

impl Bernoulli {
    /// Construct a new `Bernoulli` with the given probability of success `p`.
    ///
    /// # Precision
    ///
    /// For `p = 1.0`, the resulting distribution will always generate true.
    /// For `p = 0.0`, the resulting distribution will always generate false.
    ///
    /// This method is accurate for any input `p` in the range `[0, 1]` which is
    /// a multiple of 2<sup>-64</sup>. (Note that not all multiples of
    /// 2<sup>-64</sup> in `[0, 1]` can be represented as a `f64`.)
    #[inline]
    pub fn new(p: f64) -> Result<Bernoulli, BernoulliError> {
        if !(0.0..1.0).contains(&p) {
            if p == 1.0 {
                return Ok(Bernoulli { p_int: ALWAYS_TRUE });
            }
            return Err(BernoulliError::InvalidProbability);
        }
        Ok(Bernoulli {
            p_int: (p * SCALE) as u64,
        })
    }

    /// Construct a new `Bernoulli` with the probability of success of
    /// `numerator`-in-`denominator`. I.e. `new_ratio(2, 3)` will return
    /// a `Bernoulli` with a 2-in-3 chance, or about 67%, of returning `true`.
    ///
    /// return `true`. If `numerator == 0` it will always return `false`.
    /// For `numerator > denominator` and `denominator == 0`, this returns an
    /// error. Otherwise, for `numerator == denominator`, samples are always
    /// true; for `numerator == 0` samples are always false.
    #[inline]
    pub fn from_ratio(numerator: u32, denominator: u32) -> Result<Bernoulli, BernoulliError> {
        if numerator > denominator || denominator == 0 {
            return Err(BernoulliError::InvalidProbability);
        }
        if numerator == denominator {
            return Ok(Bernoulli { p_int: ALWAYS_TRUE });
        }
        let p_int = ((f64::from(numerator) / f64::from(denominator)) * SCALE) as u64;
        Ok(Bernoulli { p_int })
    }

    #[inline]
    /// Returns the probability (`p`) of the distribution.
    ///
    /// This value may differ slightly from the input due to loss of precision.
    pub fn p(&self) -> f64 {
        if self.p_int == ALWAYS_TRUE {
            1.0
        } else {
            (self.p_int as f64) / SCALE
        }
    }
}

impl Distribution<bool> for Bernoulli {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
        // Make sure to always return true for p = 1.0.
        if self.p_int == ALWAYS_TRUE {
            return true;
        }
        let v: u64 = rng.random();
        v < self.p_int
    }
}

#[cfg(test)]
mod test {
    use super::Bernoulli;
    use crate::distr::Distribution;
    use crate::Rng;

    #[test]
    #[cfg(feature = "serde")]
    fn test_serializing_deserializing_bernoulli() {
        let coin_flip = Bernoulli::new(0.5).unwrap();
        let de_coin_flip: Bernoulli =
            bincode::deserialize(&bincode::serialize(&coin_flip).unwrap()).unwrap();

        assert_eq!(coin_flip.p_int, de_coin_flip.p_int);
    }

    #[test]
    fn test_trivial() {
        // We prefer to be explicit here.
        #![allow(clippy::bool_assert_comparison)]

        let mut r = crate::test::rng(1);
        let always_false = Bernoulli::new(0.0).unwrap();
        let always_true = Bernoulli::new(1.0).unwrap();
        for _ in 0..5 {
            assert_eq!(r.sample::<bool, _>(&always_false), false);
            assert_eq!(r.sample::<bool, _>(&always_true), true);
            assert_eq!(Distribution::<bool>::sample(&always_false, &mut r), false);
            assert_eq!(Distribution::<bool>::sample(&always_true, &mut r), true);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)] // Miri is too slow
    fn test_average() {
        const P: f64 = 0.3;
        const NUM: u32 = 3;
        const DENOM: u32 = 10;
        let d1 = Bernoulli::new(P).unwrap();
        let d2 = Bernoulli::from_ratio(NUM, DENOM).unwrap();
        const N: u32 = 100_000;

        let mut sum1: u32 = 0;
        let mut sum2: u32 = 0;
        let mut rng = crate::test::rng(2);
        for _ in 0..N {
            if d1.sample(&mut rng) {
                sum1 += 1;
            }
            if d2.sample(&mut rng) {
                sum2 += 1;
            }
        }
        let avg1 = (sum1 as f64) / (N as f64);
        assert!((avg1 - P).abs() < 5e-3);

        let avg2 = (sum2 as f64) / (N as f64);
        assert!((avg2 - (NUM as f64) / (DENOM as f64)).abs() < 5e-3);
    }

    #[test]
    fn value_stability() {
        let mut rng = crate::test::rng(3);
        let distr = Bernoulli::new(0.4532).unwrap();
        let mut buf = [false; 10];
        for x in &mut buf {
            *x = rng.sample(distr);
        }
        assert_eq!(
            buf,
            [true, false, false, true, false, false, true, true, true, true]
        );
    }

    #[test]
    fn bernoulli_distributions_can_be_compared() {
        assert_eq!(Bernoulli::new(1.0), Bernoulli::new(1.0));
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
	use rand_core::RngCore;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
    let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
    let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut error_1_ref_0: &distr::weighted::Error = &mut error_1;
    let mut u64_0: u64 = 3985u64;
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
    let mut error_2_ref_0: &distr::weighted::Error = &mut error_2;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::thread_rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut u64_1: u64 = 9319u64;
    let mut bernoulli_0: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_1};
    let mut bernoulli_0_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_0;
    let mut u64_2: u64 = 8206u64;
    let mut bernoulli_1: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_2};
    let mut bernoulli_1_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_1;
    let mut error_3: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_3_ref_0: &distr::uniform::Error = &mut error_3;
    let mut u32_0: u32 = 2916u32;
    let mut u64_3: u64 = 8396u64;
    let mut u32_1: u32 = 1086u32;
    let mut u64_4: u64 = 186u64;
    let mut tuple_0: () = crate::distr::uniform::Error::assert_receiver_is_total_eq(error_3_ref_0);
    let mut bool_0: bool = crate::distr::bernoulli::Bernoulli::eq(bernoulli_1_ref_0, bernoulli_0_ref_0);
    let mut u64_5: u64 = crate::rngs::thread::ThreadRng::next_u64(threadrng_0_ref_0);
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut tuple_1: () = crate::distr::weighted::Error::assert_receiver_is_total_eq(error_2_ref_0);
    let mut alphanumeric_0: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::default();
    let mut bernoulli_2: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_0};
    let mut openclosed01_0: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::default();
    let mut alphabetic_0: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
    let mut bool_1: bool = crate::distr::weighted::Error::eq(error_1_ref_0, error_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_15() {
    rusty_monitor::set_test_id(15);
    let mut usize_0: usize = 6567usize;
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
    let mut u32_0: u32 = 9970u32;
    let mut u32_1: u32 = 7365u32;
    let mut u64_0: u64 = 1477u64;
    let mut u64_1: u64 = 8078u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_0;
    let mut u64_2: u64 = 1171u64;
    let mut bernoulli_0: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_2};
    let mut bernoulli_0_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_0;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut u64_3: u64 = 5433u64;
    let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
    let mut error_1_ref_0: &distr::weighted::Error = &mut error_1;
    let mut u64_4: u64 = 2477u64;
    let mut bernoulli_1: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_4};
    let mut bernoulli_1_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_1;
    let mut f64_0: f64 = crate::distr::bernoulli::Bernoulli::p(bernoulli_1_ref_0);
    let mut bernoulli_2: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_3};
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut bernoulli_2_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_2;
    let mut bool_0: bool = crate::distr::bernoulli::Bernoulli::eq(bernoulli_2_ref_0, bernoulli_0_ref_0);
    let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut u32_2: u32 = crate::rngs::mock::StepRng::next_u32(steprng_0_ref_0);
    let mut result_0: std::result::Result<crate::distr::bernoulli::Bernoulli, distr::bernoulli::BernoulliError> = crate::distr::bernoulli::Bernoulli::from_ratio(u32_1, u32_0);
    let mut threadrng_1_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_1;
    let mut u32_3: u32 = crate::rngs::thread::ThreadRng::next_u32(threadrng_1_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut u64_0: u64 = 5229u64;
    let mut bernoulli_0: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_0};
    let mut bernoulli_0_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_0;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_1_ref_0: &distr::uniform::Error = &mut error_1;
    let mut openclosed01_0: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::default();
    let mut openclosed01_0_ref_0: &crate::distr::float::OpenClosed01 = &mut openclosed01_0;
    let mut u32_0: u32 = 6429u32;
    let mut u32_1: u32 = 2667u32;
    let mut u32_2: u32 = 1552u32;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::thread_rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut u32_3: u32 = 6621u32;
    let mut u64_1: u64 = 4672u64;
    let mut u64_2: u64 = 9350u64;
    let mut u32_4: u32 = 1698u32;
    let mut u32_5: u32 = 8142u32;
    let mut u64_3: u64 = 5423u64;
    let mut u64_4: u64 = 2786u64;
    let mut u64_5: u64 = crate::rngs::thread::ThreadRng::next_u64(threadrng_0_ref_0);
    let mut error_2: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_2_ref_0: &distr::uniform::Error = &mut error_2;
    let mut bool_0: bool = crate::distr::uniform::Error::eq(error_2_ref_0, error_1_ref_0);
    let mut alphabetic_0: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
    let mut alphabetic_0_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_0;
    let mut open01_1: crate::distr::float::Open01 = crate::distr::float::Open01::clone(open01_0_ref_0);
    let mut bernoulli_1: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli::clone(bernoulli_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::default();
    let mut threadrng_0_ref_0: &crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut u64_0: u64 = 4993u64;
    let mut u64_1: u64 = 9120u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &crate::rngs::mock::StepRng = &mut steprng_0;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
    let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut error_1_ref_0: &distr::weighted::Error = &mut error_1;
    let mut error_2: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_2_ref_0: &distr::uniform::Error = &mut error_2;
    let mut error_3: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_3_ref_0: &distr::uniform::Error = &mut error_3;
    let mut error_4: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut error_4_ref_0: &distr::weighted::Error = &mut error_4;
    let mut u64_2: u64 = 6265u64;
    let mut bernoulli_0: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_2};
    let mut bernoulli_0_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_0;
    let mut isize_0: isize = -10023isize;
    let mut isize_1: isize = 4190isize;
    let mut u64_3: u64 = 8200u64;
    let mut u32_0: u32 = 9843u32;
    let mut u64_4: u64 = 5941u64;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut tuple_0: () = crate::distr::bernoulli::BernoulliError::assert_receiver_is_total_eq(bernoullierror_0_ref_0);
    let mut bool_0: bool = crate::distr::uniform::Error::eq(error_3_ref_0, error_2_ref_0);
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut tuple_1: () = crate::rngs::mock::StepRng::assert_receiver_is_total_eq(steprng_0_ref_0);
    let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::clone(threadrng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_34() {
    rusty_monitor::set_test_id(34);
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_0_ref_0: &crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_1_ref_0: &distr::uniform::Error = &mut error_1;
    let mut error_2: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_2_ref_0: &distr::uniform::Error = &mut error_2;
    let mut u64_0: u64 = 826u64;
    let mut bernoulli_0: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_0};
    let mut bernoulli_0_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_0;
    let mut u32_0: u32 = 4584u32;
    let mut u32_1: u32 = 8557u32;
    let mut error_3: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut error_3_ref_0: &distr::weighted::Error = &mut error_3;
    let mut error_4: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_4_ref_0: &distr::uniform::Error = &mut error_4;
    let mut result_0: std::result::Result<crate::distr::bernoulli::Bernoulli, distr::bernoulli::BernoulliError> = crate::distr::bernoulli::Bernoulli::from_ratio(u32_1, u32_0);
    let mut bernoulli_1: crate::distr::bernoulli::Bernoulli = std::result::Result::unwrap(result_0);
    let mut bernoulli_1_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_1;
    let mut bool_0: bool = crate::distr::bernoulli::Bernoulli::eq(bernoulli_1_ref_0, bernoulli_0_ref_0);
    let mut alphanumeric_0: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::default();
    let mut error_5: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut alphanumeric_0_ref_0: &crate::distr::other::Alphanumeric = &mut alphanumeric_0;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut error_5_ref_0: &distr::uniform::Error = &mut error_5;
    let mut tuple_0: () = crate::distr::uniform::Error::assert_receiver_is_total_eq(error_5_ref_0);
    let mut bool_1: bool = crate::distr::uniform::Error::eq(error_2_ref_0, error_1_ref_0);
    let mut error_6: distr::weighted::Error = crate::distr::weighted::Error::clone(error_0_ref_0);
    let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::clone(threadrng_0_ref_0);
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_35() {
//     rusty_monitor::set_test_id(35);
//     let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
//     let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
//     let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
//     let mut error_1_ref_0: &distr::weighted::Error = &mut error_1;
//     let mut u32_0: u32 = 3926u32;
//     let mut u32_1: u32 = 9938u32;
//     let mut standarduniform_0: crate::distr::StandardUniform = crate::distr::StandardUniform::default();
//     let mut standarduniform_0_ref_0: &crate::distr::StandardUniform = &mut standarduniform_0;
//     let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
//     let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
//     let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
//     let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
//     let mut u32_2: u32 = 9308u32;
//     let mut u32_3: u32 = 572u32;
//     let mut u32_4: u32 = 3253u32;
//     let mut u32_5: u32 = 7070u32;
//     let mut u64_0: u64 = 1778u64;
//     let mut bernoulli_0: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_0};
//     let mut bernoulli_0_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_0;
//     let mut u64_1: u64 = 1199u64;
//     let mut result_0: std::result::Result<u64, distr::uniform::Error> = std::result::Result::Ok(u64_1);
//     let mut error_2: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
//     let mut result_1: std::result::Result<crate::distr::uniform::float::UniformFloat, distr::uniform::Error> = std::result::Result::Err(error_2);
//     let mut bernoulli_1: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli::clone(bernoulli_0_ref_0);
//     let mut bool_0: bool = crate::random_ratio(u32_5, u32_4);
//     let mut bernoulli_1_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_1;
//     let mut uniformfloat_0: crate::distr::uniform::float::UniformFloat = std::result::Result::unwrap(result_1);
//     let mut bool_1: bool = crate::random_ratio(u32_3, u32_2);
//     let mut u64_2: u64 = std::result::Result::unwrap(result_0);
//     let mut bool_2: bool = crate::random_ratio(u32_1, u32_0);
//     let mut error_3: distr::weighted::Error = crate::distr::weighted::Error::clone(error_1_ref_0);
//     let mut error_4: distr::uniform::Error = crate::distr::uniform::Error::clone(error_0_ref_0);
//     panic!("From RustyUnit with love");
// }

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut alphabetic_0: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
    let mut alphabetic_0_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_0;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::thread_rng();
    let mut threadrng_0_ref_0: &crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut u64_0: u64 = 6527u64;
    let mut u64_1: u64 = 9736u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &crate::rngs::mock::StepRng = &mut steprng_0;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
    let mut u64_2: u64 = 633u64;
    let mut u64_3: u64 = 283u64;
    let mut steprng_1: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_3, u64_2);
    let mut steprng_1_ref_0: &crate::rngs::mock::StepRng = &mut steprng_1;
    let mut u64_4: u64 = 5974u64;
    let mut bernoulli_0: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_4};
    let mut bernoulli_0_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_0;
    let mut standarduniform_0: crate::distr::StandardUniform = crate::distr::StandardUniform::default();
    let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
    let mut error_1_ref_0: &distr::weighted::Error = &mut error_1;
    let mut error_2: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_3: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut bernoulli_1: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli::clone(bernoulli_0_ref_0);
    let mut error_4: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_4_ref_0: &distr::uniform::Error = &mut error_4;
    let mut steprng_2: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::clone(steprng_1_ref_0);
    let mut steprng_3: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::clone(steprng_0_ref_0);
    let mut steprng_2_ref_0: &crate::rngs::mock::StepRng = &mut steprng_2;
    let mut tuple_0: () = crate::rngs::mock::StepRng::assert_receiver_is_total_eq(steprng_2_ref_0);
    let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::clone(threadrng_0_ref_0);
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::thread_rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::default();
    let mut threadrng_1_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_1;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_1_ref_0: &distr::uniform::Error = &mut error_1;
    let mut standarduniform_0: crate::distr::StandardUniform = crate::distr::StandardUniform::default();
    let mut standarduniform_0_ref_0: &crate::distr::StandardUniform = &mut standarduniform_0;
    let mut threadrng_2: crate::rngs::thread::ThreadRng = crate::thread_rng();
    let mut threadrng_2_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_2;
    let mut usize_0: usize = 6580usize;
    let mut u64_0: u64 = 863u64;
    let mut bernoulli_0: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_0};
    let mut bernoulli_0_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_0;
    let mut u32_0: u32 = 1656u32;
    let mut u32_1: u32 = 3519u32;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut bool_0: bool = crate::random_ratio(u32_1, u32_0);
    let mut f64_0: f64 = crate::distr::bernoulli::Bernoulli::p(bernoulli_0_ref_0);
    let mut u64_1: u64 = crate::rngs::thread::ThreadRng::next_u64(threadrng_2_ref_0);
    let mut standarduniform_1: crate::distr::StandardUniform = crate::distr::StandardUniform::clone(standarduniform_0_ref_0);
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
    let mut tuple_0: () = crate::distr::uniform::Error::assert_receiver_is_total_eq(error_1_ref_0);
    let mut error_3: distr::uniform::Error = crate::distr::uniform::Error::clone(error_0_ref_0);
    let mut u64_2: u64 = crate::rngs::thread::ThreadRng::next_u64(threadrng_1_ref_0);
    let mut result_0: std::result::Result<(), rand_core::OsError> = crate::rngs::thread::ThreadRng::reseed(threadrng_0_ref_0);
    let mut error_2_ref_0: &distr::weighted::Error = &mut error_2;
    let mut error_4: distr::weighted::Error = crate::distr::weighted::Error::clone(error_2_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut u64_0: u64 = 1422u64;
    let mut u64_1: u64 = 5013u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_0;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut u64_2: u64 = 5545u64;
    let mut bernoulli_0: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_2};
    let mut bernoulli_0_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_0;
    let mut u32_0: u32 = 9463u32;
    let mut u32_1: u32 = 3207u32;
    let mut u64_3: u64 = 7439u64;
    let mut bernoulli_1: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_3};
    let mut bernoulli_1_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_1;
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
    let mut bernoulli_2: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli::clone(bernoulli_1_ref_0);
    let mut bool_0: bool = crate::random_ratio(u32_1, u32_0);
    let mut bernoulli_2_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_2;
    let mut bool_1: bool = crate::distr::bernoulli::Bernoulli::eq(bernoulli_2_ref_0, bernoulli_0_ref_0);
    let mut bernoullierror_1: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::clone(bernoullierror_0_ref_0);
    let mut openclosed01_0: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::default();
    let mut u32_2: u32 = crate::rngs::thread::ThreadRng::next_u32(threadrng_0_ref_0);
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::clone(error_0_ref_0);
    let mut alphanumeric_0: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::default();
    let mut openclosed01_0_ref_0: &crate::distr::float::OpenClosed01 = &mut openclosed01_0;
    let mut alphanumeric_0_ref_0: &crate::distr::other::Alphanumeric = &mut alphanumeric_0;
    let mut error_1_ref_0: &distr::uniform::Error = &mut error_1;
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_46() {
    rusty_monitor::set_test_id(46);
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::thread_rng();
    let mut threadrng_0_ref_0: &crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::default();
    let mut threadrng_1_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_1;
    let mut u64_0: u64 = 5063u64;
    let mut bernoulli_0: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_0};
    let mut bernoulli_0_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_0;
    let mut u64_1: u64 = 3869u64;
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
    let mut alphabetic_0: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
    let mut alphabetic_0_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_0;
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut bernoullierror_1: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_1_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_1;
    let mut bool_0: bool = crate::distr::bernoulli::BernoulliError::eq(bernoullierror_1_ref_0, bernoullierror_0_ref_0);
    let mut alphabetic_1: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::clone(alphabetic_0_ref_0);
    let mut bernoulli_1: crate::distr::bernoulli::Bernoulli = crate::distr::bernoulli::Bernoulli {p_int: u64_1};
    let mut bernoulli_1_ref_0: &crate::distr::bernoulli::Bernoulli = &mut bernoulli_1;
    let mut bool_1: bool = crate::distr::bernoulli::Bernoulli::eq(bernoulli_1_ref_0, bernoulli_0_ref_0);
    let mut alphabetic_1_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_1;
    let mut alphabetic_2: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::clone(alphabetic_1_ref_0);
    let mut result_0: std::result::Result<(), rand_core::OsError> = crate::rngs::thread::ThreadRng::reseed(threadrng_1_ref_0);
    let mut alphabetic_2_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_2;
    let mut tuple_0: () = std::result::Result::unwrap(result_0);
    let mut threadrng_2: crate::rngs::thread::ThreadRng = crate::rngs::thread::rng();
    let mut threadrng_2_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_2;
    let mut u32_0: u32 = crate::rngs::thread::ThreadRng::next_u32(threadrng_2_ref_0);
    panic!("From RustyUnit with love");
}
}