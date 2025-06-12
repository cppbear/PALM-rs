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
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    #[inline]
    fn next_u64(&mut self) -> u64 {
        let res = output_dxsm(self.state);
        self.step();
        res
    }
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {}
}
