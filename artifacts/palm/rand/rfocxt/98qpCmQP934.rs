use super::{Error, Weight};
use crate::distr::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use crate::distr::Distribution;
use crate::Rng;
use alloc::vec::Vec;
use core::fmt::{self, Debug};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
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
    {}
    pub fn update_weights(&mut self, new_weights: &[(usize, &X)]) -> Result<(), Error>
    where
        X: for<'a> core::ops::AddAssign<&'a X> + for<'a> core::ops::SubAssign<&'a X>
            + Clone + Default,
    {
        if new_weights.is_empty() {
            return Ok(());
        }
        let zero = <X as Default>::default();
        let mut total_weight = self.total_weight.clone();
        let mut prev_i = None;
        for &(i, w) in new_weights {
            if let Some(old_i) = prev_i {
                if old_i >= i {
                    return Err(Error::InvalidInput);
                }
            }
            if !(*w >= zero) {
                return Err(Error::InvalidWeight);
            }
            if i > self.cumulative_weights.len() {
                return Err(Error::InvalidInput);
            }
            let mut old_w = if i < self.cumulative_weights.len() {
                self.cumulative_weights[i].clone()
            } else {
                self.total_weight.clone()
            };
            if i > 0 {
                old_w -= &self.cumulative_weights[i - 1];
            }
            total_weight -= &old_w;
            total_weight += w;
            prev_i = Some(i);
        }
        if total_weight <= zero {
            return Err(Error::InsufficientNonZero);
        }
        let mut iter = new_weights.iter();
        let mut prev_weight = zero.clone();
        let mut next_new_weight = iter.next();
        let &(first_new_index, _) = next_new_weight.unwrap();
        let mut cumulative_weight = if first_new_index > 0 {
            self.cumulative_weights[first_new_index - 1].clone()
        } else {
            zero.clone()
        };
        for i in first_new_index..self.cumulative_weights.len() {
            match next_new_weight {
                Some(&(j, w)) if i == j => {
                    cumulative_weight += w;
                    next_new_weight = iter.next();
                }
                _ => {
                    let mut tmp = self.cumulative_weights[i].clone();
                    tmp -= &prev_weight;
                    cumulative_weight += &tmp;
                }
            }
            prev_weight = cumulative_weight.clone();
            core::mem::swap(&mut prev_weight, &mut self.cumulative_weights[i]);
        }
        self.total_weight = total_weight;
        self.weight_distribution = X::Sampler::new(zero, self.total_weight.clone())
            .unwrap();
        Ok(())
    }
}
