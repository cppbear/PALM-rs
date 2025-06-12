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
    pub fn advance(&mut self, delta: u128) {
        let mut acc_mult: u128 = 1;
        let mut acc_plus: u128 = 0;
        let mut cur_mult = MULTIPLIER;
        let mut cur_plus: u128 = 0;
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
    pub fn new(state: u128) -> Self {
        Mcg128Xsl64 { state: state | 1 }
    }
}
