pub use rand_core;
pub use rand_core::{CryptoRng, RngCore, SeedableRng, TryCryptoRng, TryRngCore};
#[cfg(feature = "thread_rng")]
pub use crate::rngs::thread::rng;
pub use rng::{Fill, Rng};
#[cfg(feature = "thread_rng")]
use crate::distr::{Distribution, StandardUniform};
#[allow(unused)]
macro_rules! trace {
    ($($x:tt)*) => {
        #[cfg(feature = "log")] { log::trace!($($x)*) }
    };
}
#[allow(unused)]
macro_rules! debug {
    ($($x:tt)*) => {
        #[cfg(feature = "log")] { log::debug!($($x)*) }
    };
}
#[allow(unused)]
macro_rules! info {
    ($($x:tt)*) => {
        #[cfg(feature = "log")] { log::info!($($x)*) }
    };
}
#[allow(unused)]
macro_rules! warn {
    ($($x:tt)*) => {
        #[cfg(feature = "log")] { log::warn!($($x)*) }
    };
}
#[allow(unused)]
macro_rules! error {
    ($($x:tt)*) => {
        #[cfg(feature = "log")] { log::error!($($x)*) }
    };
}
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
    fn random_bool(&mut self, p: f64) -> bool {
        match distr::Bernoulli::new(p) {
            Ok(d) => self.sample(d),
            Err(_) => panic!("p={:?} is outside range [0.0, 1.0]", p),
        }
    }
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
#[derive(Clone)]
pub struct ThreadRng {
    rng: Rc<UnsafeCell<ReseedingRng<Core, OsRng>>>,
}
#[cfg(feature = "thread_rng")]
#[inline]
#[track_caller]
pub fn random_bool(p: f64) -> bool {
    rng().random_bool(p)
}
pub fn rng() -> ThreadRng {
    let rng = THREAD_RNG_KEY.with(|t| t.clone());
    ThreadRng { rng }
}
