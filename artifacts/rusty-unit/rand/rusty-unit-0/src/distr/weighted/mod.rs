// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Weighted (index) sampling
//!
//! Primarily, this module houses the [`WeightedIndex`] distribution.
//! See also [`rand_distr::weighted`] for alternative implementations supporting
//! potentially-faster sampling or a more easily modifiable tree structure.
//!
//! [`rand_distr::weighted`]: https://docs.rs/rand_distr/latest/rand_distr/weighted/index.html

use core::fmt;
mod weighted_index;

pub use weighted_index::WeightedIndex;

/// Bounds on a weight
///
/// See usage in [`WeightedIndex`].
pub trait Weight: Clone {
    /// Representation of 0
    const ZERO: Self;

    /// Checked addition
    ///
    /// -   `Result::Ok`: On success, `v` is added to `self`
    /// -   `Result::Err`: Returns an error when `Self` cannot represent the
    ///     result of `self + v` (i.e. overflow). The value of `self` should be
    ///     discarded.
    #[allow(clippy::result_unit_err)]
    fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()>;
}

macro_rules! impl_weight_int {
    ($t:ty) => {
        impl Weight for $t {
            const ZERO: Self = 0;
            fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
                match self.checked_add(*v) {
                    Some(sum) => {
                        *self = sum;
                        Ok(())
                    }
                    None => Err(()),
                }
            }
        }
    };
    ($t:ty, $($tt:ty),*) => {
        impl_weight_int!($t);
        impl_weight_int!($($tt),*);
    }
}
impl_weight_int!(i8, i16, i32, i64, i128, isize);
impl_weight_int!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_weight_float {
    ($t:ty) => {
        impl Weight for $t {
            const ZERO: Self = 0.0;

            fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
                // Floats have an explicit representation for overflow
                *self += *v;
                Ok(())
            }
        }
    };
}
impl_weight_float!(f32);
impl_weight_float!(f64);

/// Invalid weight errors
///
/// This type represents errors from [`WeightedIndex::new`],
/// [`WeightedIndex::update_weights`] and other weighted distributions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Marked non_exhaustive to allow a new error code in the solution to #1476.
#[non_exhaustive]
pub enum Error {
    /// The input weight sequence is empty, too long, or wrongly ordered
    InvalidInput,

    /// A weight is negative, too large for the distribution, or not a valid number
    InvalidWeight,

    /// Not enough non-zero weights are available to sample values
    ///
    /// When attempting to sample a single value this implies that all weights
    /// are zero. When attempting to sample `amount` values this implies that
    /// less than `amount` weights are greater than zero.
    InsufficientNonZero,

    /// Overflow when calculating the sum of weights
    Overflow,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Error::InvalidInput => "Weights sequence is empty/too long/unordered",
            Error::InvalidWeight => "A weight is negative, too large or not a valid number",
            Error::InsufficientNonZero => "Not enough weights > zero",
            Error::Overflow => "Overflow when summing weights",
        })
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::clone::Clone;
	use std::cmp::PartialEq;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut bernoullierror_0: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_0_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_0;
    let mut u32_0: u32 = 7197u32;
    let mut u64_0: u64 = 5500u64;
    let mut u32_1: u32 = 6556u32;
    let mut u64_1: u64 = 9426u64;
    let mut error_0: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_0_ref_0: &distr::uniform::Error = &mut error_0;
    let mut error_1: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_1_ref_0: &distr::uniform::Error = &mut error_1;
    let mut error_2: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_2_ref_0: &distr::uniform::Error = &mut error_2;
    let mut error_3: distr::uniform::Error = crate::distr::uniform::Error::NonFinite;
    let mut error_3_ref_0: &distr::uniform::Error = &mut error_3;
    let mut error_4: distr::uniform::Error = crate::distr::uniform::Error::EmptyRange;
    let mut error_4_ref_0: &distr::uniform::Error = &mut error_4;
    let mut bernoullierror_1: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::InvalidProbability;
    let mut bernoullierror_1_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_1;
    let mut error_5: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
    let mut error_5_ref_0: &distr::weighted::Error = &mut error_5;
    let mut error_6: distr::weighted::Error = crate::distr::weighted::Error::InvalidWeight;
    let mut error_7: distr::uniform::Error = crate::distr::uniform::Error::clone(error_4_ref_0);
    let mut error_7_ref_0: &distr::uniform::Error = &mut error_7;
    let mut error_8: distr::uniform::Error = crate::distr::uniform::Error::clone(error_7_ref_0);
    let mut bool_0: bool = crate::distr::uniform::Error::eq(error_3_ref_0, error_2_ref_0);
    let mut bool_1: bool = crate::distr::uniform::Error::eq(error_1_ref_0, error_0_ref_0);
    let mut error_8_ref_0: &distr::uniform::Error = &mut error_8;
    let mut error_6_ref_0: &distr::weighted::Error = &mut error_6;
    let mut bernoullierror_2: distr::bernoulli::BernoulliError = crate::distr::bernoulli::BernoulliError::clone(bernoullierror_0_ref_0);
    let mut bernoullierror_2_ref_0: &distr::bernoulli::BernoulliError = &mut bernoullierror_2;
    panic!("From RustyUnit with love");
}
}