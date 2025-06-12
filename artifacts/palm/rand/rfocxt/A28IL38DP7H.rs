use core::cell::UnsafeCell;
use std::fmt;
use std::rc::Rc;
use std::thread_local;
use rand_core::{CryptoRng, RngCore};
use super::std::Core;
use crate::rngs::OsRng;
use crate::rngs::ReseedingRng;
const THREAD_RNG_RESEED_THRESHOLD: u64 = 1024 * 64;
thread_local!(
    static THREAD_RNG_KEY : Rc < UnsafeCell < ReseedingRng < Core, OsRng >>> = { let rng
    = ReseedingRng::new(THREAD_RNG_RESEED_THRESHOLD, OsRng).unwrap_or_else(| err |
    panic!("could not initialize ThreadRng: {}", err)); Rc::new(UnsafeCell::new(rng)) }
);
#[derive(Clone)]
pub struct ThreadRng {
    rng: Rc<UnsafeCell<ReseedingRng<Core, OsRng>>>,
}
#[derive(Debug)]
pub struct ReseedingRng<R, Rsdr>(
    BlockRng<ReseedingCore<R, Rsdr>>,
)
where
    R: BlockRngCore + SeedableRng,
    Rsdr: TryRngCore;
impl fmt::Debug for ThreadRng {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "ThreadRng {{ .. }}")
    }
}
