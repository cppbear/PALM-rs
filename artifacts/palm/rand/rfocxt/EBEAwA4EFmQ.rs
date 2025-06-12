use rand_core::{impls, RngCore};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deprecated(since = "0.9.2", note = "Deprecated without replacement")]
pub struct StepRng {
    v: u64,
    a: u64,
}
impl RngCore for StepRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    #[inline]
    fn next_u64(&mut self) -> u64 {
        let res = self.v;
        self.v = self.v.wrapping_add(self.a);
        res
    }
    #[inline]
    fn fill_bytes(&mut self, dst: &mut [u8]) {}
}
