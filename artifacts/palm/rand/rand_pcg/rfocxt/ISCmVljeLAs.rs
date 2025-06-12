pub type Pcg32 = Lcg64Xsh32;
use core::fmt;
use rand_core::{impls, le, RngCore, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
const MULTIPLIER: u64 = 6364136223846793005;
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Lcg64Xsh32 {
    state: u64,
    increment: u64,
}
impl SeedableRng for Lcg64Xsh32 {
    type Seed = [u8; 16];
    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);
        Lcg64Xsh32::from_state_incr(seed_u64[0], seed_u64[1] | 1)
    }
}
impl Lcg64Xsh32 {
    #[inline]
    pub fn advance(&mut self, delta: u64) {}
    pub fn new(state: u64, stream: u64) -> Self {
        let increment = (stream << 1) | 1;
        Lcg64Xsh32::from_state_incr(state, increment)
    }
    #[inline]
    fn from_state_incr(state: u64, increment: u64) -> Self {
        let mut pcg = Lcg64Xsh32 { state, increment };
        pcg.state = pcg.state.wrapping_add(pcg.increment);
        pcg.step();
        pcg
    }
    #[inline]
    fn step(&mut self) {}
}
