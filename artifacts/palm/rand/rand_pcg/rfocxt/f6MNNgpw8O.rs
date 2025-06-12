pub type Pcg64 = Lcg128Xsl64;
pub type Pcg64Mcg = Mcg128Xsl64;
use core::fmt;
use rand_core::{impls, le, RngCore, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
const MULTIPLIER: u128 = 0x2360_ED05_1FC6_5DA4_4385_DF64_9FCC_F645;
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mcg128Xsl64 {
    state: u128,
}
impl RngCore for Mcg128Xsl64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {}
    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(MULTIPLIER);
        output_xsl_rr(self.state)
    }
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {}
}
#[inline(always)]
fn output_xsl_rr(state: u128) -> u64 {
    const XSHIFT: u32 = 64;
    const ROTATE: u32 = 122;
    let rot = (state >> ROTATE) as u32;
    let xsl = ((state >> XSHIFT) as u64) ^ (state as u64);
    xsl.rotate_right(rot)
}
