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
impl SeedableRng for SmallRng {
    type Seed = [u8; 32];
    #[inline(always)]
    fn from_seed(seed: Self::Seed) -> Self {
        const LEN: usize = core::mem::size_of::<<Rng as SeedableRng>::Seed>();
        let seed = (&seed[..LEN]).try_into().unwrap();
        SmallRng(Rng::from_seed(seed))
    }
    #[inline(always)]
    fn seed_from_u64(state: u64) -> Self {
        SmallRng(Rng::seed_from_u64(state))
    }
}
impl SeedableRng for Xoshiro256PlusPlus {
    type Seed = [u8; 32];
    #[inline]
    fn from_seed(seed: [u8; 32]) -> Xoshiro256PlusPlus {
        let mut state = [0; 4];
        read_u64_into(&seed, &mut state);
        if state.iter().all(|&x| x == 0) {
            return Self::seed_from_u64(0);
        }
        Xoshiro256PlusPlus { s: state }
    }
    #[inline]
    fn seed_from_u64(mut state: u64) -> Self {
        const PHI: u64 = 0x9e3779b97f4a7c15;
        let mut s = [0; 4];
        for i in s.iter_mut() {
            state = state.wrapping_add(PHI);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z = z ^ (z >> 31);
            *i = z;
        }
        debug_assert_ne!(s, [0; 4]);
        Xoshiro256PlusPlus { s }
    }
}
