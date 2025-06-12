pub use float::UniformFloat;
pub use int::{UniformInt, UniformUsize};
pub use other::{UniformChar, UniformDuration};
use core::fmt;
use core::ops::{Range, RangeInclusive, RangeTo, RangeToInclusive};
use crate::distr::Distribution;
use crate::{Rng, RngCore};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
macro_rules! impl_sample_range_u {
    ($t:ty) => {
        impl SampleRange <$t > for RangeTo <$t > { #[inline] fn sample_single < R :
        RngCore + ? Sized > (self, rng : & mut R) -> Result <$t, Error > { <$t as
        SampleUniform >::Sampler::sample_single(0, self.end, rng) } #[inline] fn
        is_empty(& self) -> bool { 0 == self.end } } impl SampleRange <$t > for
        RangeToInclusive <$t > { #[inline] fn sample_single < R : RngCore + ? Sized >
        (self, rng : & mut R) -> Result <$t, Error > { <$t as SampleUniform
        >::Sampler::sample_single_inclusive(0, self.end, rng) } #[inline] fn is_empty(&
        self) -> bool { false } }
    };
}
impl_sample_range_u!(u8);
impl_sample_range_u!(u16);
impl_sample_range_u!(u32);
impl_sample_range_u!(u64);
impl_sample_range_u!(u128);
impl_sample_range_u!(usize);
#[cfg(feature = "alloc")]
pub trait SampleString {
    fn append_string<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        string: &mut String,
        len: usize,
    );
    #[inline]
    fn sample_string<R: Rng + ?Sized>(&self, rng: &mut R, len: usize) -> String;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "X::Sampler: Serialize")))]
#[cfg_attr(
    feature = "serde",
    serde(bound(deserialize = "X::Sampler: Deserialize<'de>"))
)]
pub struct Uniform<X: SampleUniform>(X::Sampler);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `low > high`, or equal in case of exclusive range.
    EmptyRange,
    /// Input or range `high - low` is non-finite. Not relevant to integer types.
    NonFinite,
}
impl<X: SampleUniform> Uniform<X> {
    pub fn new<B1, B2>(low: B1, high: B2) -> Result<Uniform<X>, Error>
    where
        B1: SampleBorrow<X> + Sized,
        B2: SampleBorrow<X> + Sized,
    {
        X::Sampler::new(low, high).map(Uniform)
    }
    pub fn new_inclusive<B1, B2>(low: B1, high: B2) -> Result<Uniform<X>, Error>
    where
        B1: SampleBorrow<X> + Sized,
        B2: SampleBorrow<X> + Sized,
    {}
}
