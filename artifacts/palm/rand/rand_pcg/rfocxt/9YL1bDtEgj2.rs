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
    pub fn advance(&mut self, delta: u128) {
        let mut acc_mult: u128 = 1;
        let mut acc_plus: u128 = 0;
        let mut cur_mult = MULTIPLIER as u128;
        let mut cur_plus = self.increment;
        let mut mdelta = delta;
        while mdelta > 0 {
            if (mdelta & 1) != 0 {
                acc_mult = acc_mult.wrapping_mul(cur_mult);
                acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
            }
            cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
            cur_mult = cur_mult.wrapping_mul(cur_mult);
            mdelta /= 2;
        }
        self.state = acc_mult.wrapping_mul(self.state).wrapping_add(acc_plus);
    }
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
