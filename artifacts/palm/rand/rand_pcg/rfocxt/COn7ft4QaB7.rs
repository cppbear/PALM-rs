pub type Pcg64 = Lcg128Xsl64;
pub type Pcg64Mcg = Mcg128Xsl64;
use core::fmt;
use rand_core::{impls, le, RngCore, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
const MULTIPLIER: u128 = 0x2360_ED05_1FC6_5DA4_4385_DF64_9FCC_F645;
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Lcg128Xsl64 {
    state: u128,
    increment: u128,
}
impl RngCore for Lcg128Xsl64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {}
    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.step();
        output_xsl_rr(self.state)
    }
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {}
}
impl Lcg128Xsl64 {
    #[inline]
    pub fn advance(&mut self, delta: u128) {}
    pub fn new(state: u128, stream: u128) -> Self {
        let increment = (stream << 1) | 1;
        Lcg128Xsl64::from_state_incr(state, increment)
    }
    #[inline]
    fn from_state_incr(state: u128, increment: u128) -> Self {
        let mut pcg = Lcg128Xsl64 { state, increment };
        pcg.state = pcg.state.wrapping_add(pcg.increment);
        pcg.step();
        pcg
    }
    #[inline]
    fn step(&mut self) {
        self.state = self.state.wrapping_mul(MULTIPLIER).wrapping_add(self.increment);
    }
}
#[inline(always)]
fn output_xsl_rr(state: u128) -> u64 {
    const XSHIFT: u32 = 64;
    const ROTATE: u32 = 122;
    let rot = (state >> ROTATE) as u32;
    let xsl = ((state >> XSHIFT) as u64) ^ (state as u64);
    xsl.rotate_right(rot)
}
