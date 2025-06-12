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
mod tests_llm_16_245 {
    use super::*;

use crate::*;
    use crate::distr::weighted::Weight;

    #[test]
    fn test_checked_add_assign() {
        let mut a: f32 = 1.0;
        let b: f32 = 2.0;
        
        let result = a.checked_add_assign(&b);
        assert!(result.is_ok());
        assert_eq!(a, 3.0);
    }

    #[test]
    fn test_checked_add_assign_negative() {
        let mut a: f32 = f32::INFINITY;
        let b: f32 = 1.0;
        
        let result = a.checked_add_assign(&b);
        assert!(result.is_ok());
        assert_eq!(a, f32::INFINITY);
    }

    #[test]
    fn test_checked_add_assign_overflow() {
        let mut a: f32 = f32::MAX;
        let b: f32 = 1.0;

        let result = a.checked_add_assign(&b);
        assert!(result.is_ok());
        assert_eq!(a, f32::MAX);
    }
}

#[cfg(test)]
mod tests_llm_16_252 {
    use super::*;

use crate::*;
    use crate::distr::weighted::Weight;

    #[test]
    fn test_checked_add_assign() {
        let mut weight1: f64 = 1.0;
        let weight2: f64 = 2.0;

        let result = weight1.checked_add_assign(&weight2);
        assert!(result.is_ok());
        assert_eq!(weight1, 3.0);
    }

    #[test]
    fn test_checked_add_assign_overflow() {
        let mut weight1: f64 = f64::MAX;
        let weight2: f64 = 1.0;

        let result = weight1.checked_add_assign(&weight2);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tests_llm_16_255 {
    use super::*;

use crate::*;
    use crate::distr::weighted::Weight;

    #[test]
    fn test_checked_add_assign_success() {
        let mut a: i32 = 5;
        let b: i32 = 3;
        let result = a.checked_add_assign(&b);
        assert_eq!(result, Ok(()));
        assert_eq!(a, 8);
    }

    #[test]
    fn test_checked_add_assign_overflow() {
        let mut a: i32 = i32::MAX;
        let b: i32 = 1;
        let result = a.checked_add_assign(&b);
        assert_eq!(result, Err(()));
        assert_eq!(a, i32::MAX);
    }

    #[test]
    fn test_checked_add_assign_negative() {
        let mut a: i32 = 0;
        let b: i32 = -1;
        let result = a.checked_add_assign(&b);
        assert_eq!(result, Err(()));
        assert_eq!(a, 0);
    }

    #[test]
    fn test_checked_add_assign_zero() {
        let mut a: i32 = 10;
        let b: i32 = 0;
        let result = a.checked_add_assign(&b);
        assert_eq!(result, Ok(()));
        assert_eq!(a, 10);
    }
}

#[cfg(test)]
mod tests_llm_16_256 {
    use super::*;

use crate::*;
    use crate::distr::weighted::Weight;

    #[test]
    fn test_checked_add_assign_success() {
        let mut a: i64 = 10;
        let b: i64 = 20;
        let result = a.checked_add_assign(&b);
        assert_eq!(result, Ok(()));
        assert_eq!(a, 30);
    }

    #[test]
    fn test_checked_add_assign_overflow() {
        let mut a: i64 = i64::MAX;
        let b: i64 = 1;
        let result = a.checked_add_assign(&b);
        assert_eq!(result, Err(()));
        assert_eq!(a, i64::MAX);
    }

    #[test]
    fn test_checked_add_assign_negative() {
        let mut a: i64 = 10;
        let b: i64 = -5;
        let result = a.checked_add_assign(&b);
        assert_eq!(result, Ok(()));
        assert_eq!(a, 5);
    }

    #[test]
    fn test_checked_add_assign_zero() {
        let mut a: i64 = 10;
        let b: i64 = 0;
        let result = a.checked_add_assign(&b);
        assert_eq!(result, Ok(()));
        assert_eq!(a, 10);
    }
}

#[cfg(test)]
mod tests_llm_16_302 {
    use super::*;

use crate::*;
    use crate::distr::weighted::Weight;

    #[test]
    fn test_checked_add_assign_success() {
        let mut a: u128 = 100;
        let b: u128 = 50;

        let result = a.checked_add_assign(&b);
        assert_eq!(result, Ok(()));
        assert_eq!(a, 150);
    }

    #[test]
    fn test_checked_add_assign_overflow() {
        let mut a: u128 = u128::MAX;
        let b: u128 = 1;

        let result = a.checked_add_assign(&b);
        assert_eq!(result, Err(()));
        assert_eq!(a, u128::MAX);
    }
}

#[cfg(test)]
mod tests_llm_16_307 {
    use super::*;

use crate::*;
    use crate::distr::weighted::Weight;

    #[test]
    fn test_checked_add_assign_success() {
        let mut a: u32 = 5;
        let b: u32 = 10;
        assert_eq!(a.checked_add_assign(&b), Ok(()));
        assert_eq!(a, 15);
    }

    #[test]
    fn test_checked_add_assign_overflow() {
        let mut a: u32 = u32::MAX;
        let b: u32 = 1;
        assert_eq!(a.checked_add_assign(&b), Err(()));
        assert_eq!(a, u32::MAX);
    }

    #[test]
    fn test_checked_add_assign_zero() {
        let mut a: u32 = 10;
        let b: u32 = 0;
        assert_eq!(a.checked_add_assign(&b), Ok(()));
        assert_eq!(a, 10);
    }
}
