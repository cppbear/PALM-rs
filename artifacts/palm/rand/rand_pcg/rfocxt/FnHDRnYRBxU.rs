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
