use super::{Error, Weight};
use crate::distr::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use crate::distr::Distribution;
use crate::Rng;
use alloc::vec::Vec;
use core::fmt::{self, Debug};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
pub trait Weight: Clone {
    const ZERO: Self;
    #[allow(clippy::result_unit_err)]
    fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()>;
}
pub trait Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;
    fn sample_iter<R>(self, rng: R) -> Iter<Self, R, T>
    where
        R: Rng,
        Self: Sized,
    {
        Iter {
            distr: self,
            rng,
            phantom: core::marker::PhantomData,
        }
    }
    fn map<F, S>(self, func: F) -> Map<Self, F, T, S>
    where
        F: Fn(T) -> S,
        Self: Sized,
    {
        Map {
            distr: self,
            func,
            phantom: core::marker::PhantomData,
        }
    }
}
pub trait SampleUniform: Sized {
    type Sampler: UniformSampler<X = Self>;
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WeightedIndex<X: SampleUniform + PartialOrd> {
    cumulative_weights: Vec<X>,
    total_weight: X,
    weight_distribution: X::Sampler,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `low > high`, or equal in case of exclusive range.
    EmptyRange,
    /// Input or range `high - low` is non-finite. Not relevant to integer types.
    NonFinite,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
impl<X: SampleUniform + PartialOrd> WeightedIndex<X> {
    pub fn new<I>(weights: I) -> Result<WeightedIndex<X>, Error>
    where
        I: IntoIterator,
        I::Item: SampleBorrow<X>,
        X: Weight,
    {
        let mut iter = weights.into_iter();
        let mut total_weight: X = iter
            .next()
            .ok_or(Error::InvalidInput)?
            .borrow()
            .clone();
        let zero = X::ZERO;
        if !(total_weight >= zero) {
            return Err(Error::InvalidWeight);
        }
        let mut weights = Vec::<X>::with_capacity(iter.size_hint().0);
        for w in iter {
            if !(w.borrow() >= &zero) {
                return Err(Error::InvalidWeight);
            }
            weights.push(total_weight.clone());
            if let Err(()) = total_weight.checked_add_assign(w.borrow()) {
                return Err(Error::Overflow);
            }
        }
        if total_weight == zero {
            return Err(Error::InsufficientNonZero);
        }
        let distr = X::Sampler::new(zero, total_weight.clone()).unwrap();
        Ok(WeightedIndex {
            cumulative_weights: weights,
            total_weight,
            weight_distribution: distr,
        })
    }
    pub fn update_weights(&mut self, new_weights: &[(usize, &X)]) -> Result<(), Error>
    where
        X: for<'a> core::ops::AddAssign<&'a X> + for<'a> core::ops::SubAssign<&'a X>
            + Clone + Default,
    {}
}
