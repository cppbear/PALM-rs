// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Mock random number generator

use rand_core::{impls, RngCore};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A mock generator yielding very predictable output
///
/// This generates an arithmetic sequence (i.e. adds a constant each step)
/// over a `u64` number, using wrapping arithmetic. If the increment is 0
/// the generator yields a constant.
///
/// Other integer types (64-bit and smaller) are produced via cast from `u64`.
///
/// Other types are produced via their implementation of [`Rng`](crate::Rng) or
/// [`Distribution`](crate::distr::Distribution).
/// Output values may not be intuitive and may change in future releases but
/// are considered
/// [portable](https://rust-random.github.io/book/portability.html).
/// (`bool` output is true when bit `1u64 << 31` is set.)
///
/// # Example
///
/// ```
/// use rand::Rng;
/// use rand::rngs::mock::StepRng;
///
/// let mut my_rng = StepRng::new(2, 1);
/// let sample: [u64; 3] = my_rng.random();
/// assert_eq!(sample, [2, 3, 4]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StepRng {
    v: u64,
    a: u64,
}

impl StepRng {
    /// Create a `StepRng`, yielding an arithmetic sequence starting with
    /// `initial` and incremented by `increment` each time.
    pub fn new(initial: u64, increment: u64) -> Self {
        StepRng {
            v: initial,
            a: increment,
        }
    }
}

impl RngCore for StepRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let res = self.v;
        self.v = self.v.wrapping_add(self.a);
        res
    }

    #[inline]
    fn fill_bytes(&mut self, dst: &mut [u8]) {
        impls::fill_bytes_via_next(self, dst)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(any(feature = "alloc", feature = "serde"))]
    use super::StepRng;

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialization_step_rng() {
        let some_rng = StepRng::new(42, 7);
        let de_some_rng: StepRng =
            bincode::deserialize(&bincode::serialize(&some_rng).unwrap()).unwrap();
        assert_eq!(some_rng.v, de_some_rng.v);
        assert_eq!(some_rng.a, de_some_rng.a);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_bool() {
        use crate::{distr::StandardUniform, Rng};

        // If this result ever changes, update doc on StepRng!
        let rng = StepRng::new(0, 1 << 31);
        let result: alloc::vec::Vec<bool> = rng.sample_iter(StandardUniform).take(6).collect();
        assert_eq!(&result, &[false, true, false, true, false, true]);
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use rand_core::RngCore;
	use std::cmp::Eq;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_12() {
    rusty_monitor::set_test_id(12);
    let mut error_0: distr::weighted::Error = crate::distr::weighted::Error::Overflow;
    let mut error_0_ref_0: &distr::weighted::Error = &mut error_0;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
    let mut open01_1: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut open01_1_ref_0: &crate::distr::float::Open01 = &mut open01_1;
    let mut u32_0: u32 = 9692u32;
    let mut u32_1: u32 = 5800u32;
    let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::Overflow;
    let mut error_1_ref_0: &distr::weighted::Error = &mut error_1;
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut error_2_ref_0: &distr::weighted::Error = &mut error_2;
    let mut error_3: distr::weighted::Error = crate::distr::weighted::Error::Overflow;
    let mut error_3_ref_0: &distr::weighted::Error = &mut error_3;
    let mut u64_0: u64 = 6632u64;
    let mut u64_1: u64 = 6254u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_0;
    let mut u64_2: u64 = 1772u64;
    let mut u64_3: u64 = 3887u64;
    let mut steprng_1: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng {v: u64_3, a: u64_2};
    let mut steprng_1_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_1;
    let mut u32_2: u32 = crate::rngs::mock::StepRng::next_u32(steprng_1_ref_0);
    let mut u64_4: u64 = crate::rngs::mock::StepRng::next_u64(steprng_0_ref_0);
    let mut bool_0: bool = crate::distr::weighted::Error::eq(error_2_ref_0, error_1_ref_0);
    let mut bool_1: bool = crate::random_ratio(u32_1, u32_0);
    let mut tuple_0: () = crate::distr::bernoulli::BernoulliError::assert_receiver_is_total_eq(bernoullierror_0_ref_0);
    let mut tuple_1: () = crate::distr::weighted::Error::assert_receiver_is_total_eq(error_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_20() {
    rusty_monitor::set_test_id(20);
    let mut u64_0: u64 = 8422u64;
    let mut u64_1: u64 = 4348u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_1, u64_0);
    let mut steprng_0_ref_0: &crate::rngs::mock::StepRng = &mut steprng_0;
    let mut f64_0: f64 = -16458.282240f64;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::thread_rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut u64_2: u64 = 9383u64;
    let mut u64_3: u64 = 6914u64;
    let mut steprng_1: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_3, u64_2);
    let mut steprng_1_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_1;
    let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::rngs::thread::ThreadRng::default();
    let mut threadrng_1_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_1;
    let mut u64_4: u64 = 9277u64;
    let mut u64_5: u64 = 2837u64;
    let mut steprng_2: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng {v: u64_5, a: u64_4};
    let mut steprng_2_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_2;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut openclosed01_0: crate::distr::float::OpenClosed01 = crate::distr::float::OpenClosed01::default();
    let mut tuple_0: () = crate::distr::bernoulli::BernoulliError::assert_receiver_is_total_eq(bernoullierror_0_ref_0);
    let mut u64_6: u64 = crate::rngs::mock::StepRng::next_u64(steprng_2_ref_0);
    let mut bernoullierror_1: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_1_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_1;
    let mut tuple_1: () = crate::distr::bernoulli::BernoulliError::assert_receiver_is_total_eq(bernoullierror_1_ref_0);
    let mut u64_7: u64 = crate::rngs::mock::StepRng::next_u64(steprng_1_ref_0);
    let mut openclosed01_0_ref_0: &crate::distr::float::OpenClosed01 = &mut openclosed01_0;
    let mut error_1: distr::weighted::Error = crate::distr::weighted::Error::InsufficientNonZero;
    let mut bool_0: bool = crate::random_bool(f64_0);
    let mut error_1_ref_0: &distr::weighted::Error = &mut error_1;
    let mut steprng_3: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::clone(steprng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_21() {
    rusty_monitor::set_test_id(21);
    let mut u64_0: u64 = 4312u64;
    let mut u64_1: u64 = 8672u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng {v: u64_1, a: u64_0};
    let mut steprng_0_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_0;
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::thread_rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut alphanumeric_0: crate::distr::other::Alphanumeric = crate::distr::other::Alphanumeric::default();
    let mut alphanumeric_0_ref_0: &crate::distr::other::Alphanumeric = &mut alphanumeric_0;
    let mut threadrng_1: crate::rngs::thread::ThreadRng = crate::thread_rng();
    let mut threadrng_1_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_1;
    let mut u64_2: u64 = 4598u64;
    let mut u64_3: u64 = 6408u64;
    let mut steprng_1: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng {v: u64_3, a: u64_2};
    let mut steprng_1_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_1;
    let mut u64_4: u64 = 8693u64;
    let mut u64_5: u64 = 8969u64;
    let mut steprng_2: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_5, u64_4);
    let mut steprng_2_ref_0: &crate::rngs::mock::StepRng = &mut steprng_2;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut result_0: std::result::Result<crate::distr::uniform::int::UniformInt<i16>, distr::uniform::Error> = std::result::Result::Err(error_0);
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut result_1: std::result::Result<crate::distr::uniform::int::UniformInt<u32>, distr::uniform::Error> = std::result::Result::Err(error_1);
    let mut alphabetic_0: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::default();
    let mut alphabetic_0_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_0;
    let mut alphabetic_1: crate::distr::other::Alphabetic = crate::distr::other::Alphabetic::clone(alphabetic_0_ref_0);
    let mut alphabetic_1_ref_0: &crate::distr::other::Alphabetic = &mut alphabetic_1;
    let mut error_2: distr::weighted::Error = crate::distr::weighted::Error::InvalidInput;
    let mut u32_0: u32 = crate::rngs::mock::StepRng::next_u32(steprng_1_ref_0);
    let mut uniformint_0: crate::distr::uniform::int::UniformInt<u32> = std::result::Result::unwrap(result_1);
    let mut uniformint_1: crate::distr::uniform::int::UniformInt<i16> = std::result::Result::unwrap(result_0);
    let mut result_2: std::result::Result<(), rand_core::OsError> = crate::rngs::thread::ThreadRng::reseed(threadrng_0_ref_0);
    let mut u64_6: u64 = crate::rngs::mock::StepRng::next_u64(steprng_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_49() {
    rusty_monitor::set_test_id(49);
    let mut threadrng_0: crate::rngs::thread::ThreadRng = crate::thread_rng();
    let mut threadrng_0_ref_0: &mut crate::rngs::thread::ThreadRng = &mut threadrng_0;
    let mut u32_0: u32 = 3502u32;
    let mut u32_1: u32 = 1252u32;
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut u64_0: u64 = 614u64;
    let mut u64_1: u64 = 3043u64;
    let mut steprng_0: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng {v: u64_1, a: u64_0};
    let mut steprng_0_ref_0: &crate::rngs::mock::StepRng = &mut steprng_0;
    let mut u64_2: u64 = 2412u64;
    let mut u64_3: u64 = 4383u64;
    let mut steprng_1: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng {v: u64_3, a: u64_2};
    let mut steprng_1_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_1;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut result_0: std::result::Result<f32, distr::uniform::Error> = std::result::Result::Err(error_0);
    let mut result_0_ref_0: &std::result::Result<f32, distr::uniform::Error> = &mut result_0;
    let mut u64_4: u64 = 2733u64;
    let mut u64_5: u64 = 6734u64;
    let mut steprng_2: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::new(u64_5, u64_4);
    let mut steprng_2_ref_0: &crate::rngs::mock::StepRng = &mut steprng_2;
    let mut open01_0: crate::distr::float::Open01 = crate::distr::float::Open01::default();
    let mut open01_0_ref_0: &crate::distr::float::Open01 = &mut open01_0;
    let mut open01_1: crate::distr::float::Open01 = crate::distr::float::Open01::clone(open01_0_ref_0);
    let mut u64_6: u64 = crate::rngs::mock::StepRng::next_u64(steprng_1_ref_0);
    let mut steprng_3: crate::rngs::mock::StepRng = crate::rngs::mock::StepRng::clone(steprng_0_ref_0);
    let mut steprng_3_ref_0: &mut crate::rngs::mock::StepRng = &mut steprng_3;
    let mut result_1: std::result::Result<crate::distr::bernoulli::Bernoulli, distr::bernoulli::BernoulliError> = crate::distr::bernoulli::Bernoulli::from_ratio(u32_1, u32_0);
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut u64_7: u64 = crate::rngs::thread::ThreadRng::next_u64(threadrng_0_ref_0);
    panic!("From RustyUnit with love");
}
}