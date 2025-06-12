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
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.step();
        output_xsl_rr(self.state)
    }
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {}
}
