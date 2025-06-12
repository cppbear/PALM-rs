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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `low > high`, or equal in case of exclusive range.
    EmptyRange,
    /// Input or range `high - low` is non-finite. Not relevant to integer types.
    NonFinite,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match self {
                Error::EmptyRange => {
                    "low > high (or equal if exclusive) in uniform distribution"
                }
                Error::NonFinite => "Non-finite range in uniform distribution",
            },
        )
    }
}
