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
#[cfg(feature = "thread_rng")]
#[inline]
#[track_caller]
pub fn fill<T: Fill + ?Sized>(dest: &mut T) {
    dest.fill(&mut rng())
}
pub fn rng() -> ThreadRng {
    let rng = THREAD_RNG_KEY.with(|t| t.clone());
    ThreadRng { rng }
}
