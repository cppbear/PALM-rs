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
impl RngCore for ThreadRng {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {}
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        let rng = unsafe { &mut *self.rng.get() };
        rng.next_u64()
    }
    #[inline(always)]
    fn fill_bytes(&mut self, dest: &mut [u8]) {}
}
