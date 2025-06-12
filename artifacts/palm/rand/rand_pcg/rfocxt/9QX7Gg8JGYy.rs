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
impl RngCore for Lcg128CmDxsm64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {}
    #[inline]
    fn next_u64(&mut self) -> u64 {
        let res = output_dxsm(self.state);
        self.step();
        res
    }
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {}
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
    fn step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(MULTIPLIER as u128)
            .wrapping_add(self.increment);
    }
}
#[inline(always)]
fn output_dxsm(state: u128) -> u64 {
    let mut hi = (state >> 64) as u64;
    let mut lo = state as u64;
    lo |= 1;
    hi ^= hi >> 32;
    hi = hi.wrapping_mul(MULTIPLIER);
    hi ^= hi >> 48;
    hi = hi.wrapping_mul(lo);
    hi
}
