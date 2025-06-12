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
pub trait SampleRange<T> {
    fn sample_single<R: RngCore + ?Sized>(self, rng: &mut R) -> Result<T, Error>;
    fn is_empty(&self) -> bool;
}
pub trait SampleUniform: Sized {
    type Sampler: UniformSampler<X = Self>;
}
impl<T: SampleUniform + PartialOrd> SampleRange<T> for Range<T> {
    #[inline]
    fn sample_single<R: RngCore + ?Sized>(self, rng: &mut R) -> Result<T, Error> {}
    #[inline]
    fn is_empty(&self) -> bool {
        !(self.start < self.end)
    }
}
