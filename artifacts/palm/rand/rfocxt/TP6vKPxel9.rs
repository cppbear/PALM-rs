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
impl<X: SampleUniform + PartialOrd + Clone> WeightedIndex<X> {
    pub fn weight(&self, index: usize) -> Option<X>
    where
        X: for<'a> core::ops::SubAssign<&'a X>,
    {}
    pub fn weights(&self) -> WeightedIndexIter<'_, X>
    where
        X: for<'a> core::ops::SubAssign<&'a X>,
    {}
    pub fn total_weight(&self) -> X {
        self.total_weight.clone()
    }
}
