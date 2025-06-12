#[cfg(any(target_pointer_width = "32", target_pointer_width = "16"))]
type Rng = super::xoshiro128plusplus::Xoshiro128PlusPlus;
#[cfg(target_pointer_width = "64")]
type Rng = super::xoshiro256plusplus::Xoshiro256PlusPlus;
use rand_core::{RngCore, SeedableRng};
pub trait Rng: RngCore {
    #[inline]
    fn random<T>(&mut self) -> T
    where
        StandardUniform: Distribution<T>;
    #[inline]
    fn random_iter<T>(self) -> distr::Iter<StandardUniform, Self, T>
    where
        Self: Sized,
        StandardUniform: Distribution<T>,
    {
        StandardUniform.sample_iter(self)
    }
    #[track_caller]
    fn random_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>;
    #[inline]
    #[track_caller]
    fn random_bool(&mut self, p: f64) -> bool;
    #[inline]
    #[track_caller]
    fn random_ratio(&mut self, numerator: u32, denominator: u32) -> bool;
    fn sample<T, D: Distribution<T>>(&mut self, distr: D) -> T;
    fn sample_iter<T, D>(self, distr: D) -> distr::Iter<D, Self, T>
    where
        D: Distribution<T>,
        Self: Sized,
    {
        distr.sample_iter(self)
    }
    #[track_caller]
    fn fill<T: Fill + ?Sized>(&mut self, dest: &mut T);
    #[inline]
    #[deprecated(
        since = "0.9.0",
        note = "Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024."
    )]
    fn r#gen<T>(&mut self) -> T
    where
        StandardUniform: Distribution<T>;
    #[inline]
    #[deprecated(since = "0.9.0", note = "Renamed to `random_range`")]
    fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>;
    #[inline]
    #[deprecated(since = "0.9.0", note = "Renamed to `random_bool`")]
    fn gen_bool(&mut self, p: f64) -> bool;
    #[inline]
    #[deprecated(since = "0.9.0", note = "Renamed to `random_ratio`")]
    fn gen_ratio(&mut self, numerator: u32, denominator: u32) -> bool;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SmallRng(Rng);
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Xoshiro256PlusPlus {
    s: [u64; 4],
}
impl RngCore for SmallRng {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {}
    #[inline(always)]
    fn fill_bytes(&mut self, dest: &mut [u8]) {}
}
impl RngCore for Xoshiro256PlusPlus {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let val = self.next_u64();
        (val >> 32) as u32
    }
    #[inline]
    fn next_u64(&mut self) -> u64 {}
    #[inline]
    fn fill_bytes(&mut self, dst: &mut [u8]) {}
}
