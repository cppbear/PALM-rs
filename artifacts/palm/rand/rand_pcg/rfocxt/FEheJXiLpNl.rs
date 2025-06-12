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
impl RngCore for Lcg64Xsh32 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let state = self.state;
        self.step();
        const ROTATE: u32 = 59;
        const XSHIFT: u32 = 18;
        const SPARE: u32 = 27;
        let rot = (state >> ROTATE) as u32;
        let xsh = (((state >> XSHIFT) ^ state) >> SPARE) as u32;
        xsh.rotate_right(rot)
    }
    #[inline]
    fn next_u64(&mut self) -> u64 {}
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {}
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
    fn step(&mut self) {
        self.state = self.state.wrapping_mul(MULTIPLIER).wrapping_add(self.increment);
    }
}
