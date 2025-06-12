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
#[derive(Clone)]
pub struct ThreadRng {
    rng: Rc<UnsafeCell<ReseedingRng<Core, OsRng>>>,
}
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StandardUniform;
#[derive(Debug)]
pub struct Iter<D, R, T> {
    distr: D,
    rng: R,
    phantom: core::marker::PhantomData<T>,
}
#[cfg(feature = "thread_rng")]
#[inline]
pub fn random_iter<T>() -> distr::Iter<StandardUniform, rngs::ThreadRng, T>
where
    StandardUniform: Distribution<T>,
{
    rng().random_iter()
}
pub fn rng() -> ThreadRng {
    let rng = THREAD_RNG_KEY.with(|t| t.clone());
    ThreadRng { rng }
}
