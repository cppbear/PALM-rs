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
    fn next_u32(&mut self) -> u32 {}
    #[inline]
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {}
}
