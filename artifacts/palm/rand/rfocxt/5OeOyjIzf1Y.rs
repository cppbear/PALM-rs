use crate::distr::Distribution;
use crate::Rng;
use core::fmt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
const ALWAYS_TRUE: u64 = u64::MAX;
const SCALE: f64 = 2.0 * (1u64 << 63) as f64;
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
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bernoulli {
    /// Probability of success, relative to the maximal integer.
    p_int: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BernoulliError {
    /// `p < 0` or `p > 1`.
    InvalidProbability,
}
impl Bernoulli {
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
    #[inline]
    pub fn from_ratio(
        numerator: u32,
        denominator: u32,
    ) -> Result<Bernoulli, BernoulliError> {}
    #[inline]
    pub fn p(&self) -> f64 {}
}
