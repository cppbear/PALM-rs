use rand_core::{CryptoRng, RngCore, SeedableRng};
#[cfg(any(test, feature = "os_rng"))]
pub(crate) use rand_chacha::ChaCha12Core as Core;
use rand_chacha::ChaCha12Rng as Rng;
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
pub struct StdRng(Rng);
impl SeedableRng for StdRng {
    type Seed = [u8; 32];
    #[inline(always)]
    fn from_seed(seed: Self::Seed) -> Self {
        StdRng(Rng::from_seed(seed))
    }
}
