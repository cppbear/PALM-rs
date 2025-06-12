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
impl Mcg128Xsl64 {
    #[inline]
    pub fn advance(&mut self, delta: u128) {}
    pub fn new(state: u128) -> Self {
        Mcg128Xsl64 { state: state | 1 }
    }
}
