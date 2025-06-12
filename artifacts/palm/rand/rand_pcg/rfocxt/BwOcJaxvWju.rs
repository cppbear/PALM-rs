pub type Pcg64Dxsm = Lcg128CmDxsm64;
use core::fmt;
use rand_core::{impls, le, RngCore, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
const MULTIPLIER: u64 = 15750249268501108917;
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Lcg128CmDxsm64 {
    state: u128,
    increment: u128,
}
impl SeedableRng for Lcg128CmDxsm64 {
    type Seed = [u8; 32];
    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 4];
        le::read_u64_into(&seed, &mut seed_u64);
        let state = u128::from(seed_u64[0]) | (u128::from(seed_u64[1]) << 64);
        let incr = u128::from(seed_u64[2]) | (u128::from(seed_u64[3]) << 64);
        Self::from_state_incr(state, incr | 1)
    }
}
impl Lcg128CmDxsm64 {
    #[inline]
    pub fn advance(&mut self, delta: u128) {}
    pub fn new(state: u128, stream: u128) -> Self {
        let increment = (stream << 1) | 1;
        Self::from_state_incr(state, increment)
    }
    #[inline]
    fn from_state_incr(state: u128, increment: u128) -> Self {
        let mut pcg = Self { state, increment };
        pcg.state = pcg.state.wrapping_add(pcg.increment);
        pcg.step();
        pcg
    }
    #[inline(always)]
    fn step(&mut self) {}
}
