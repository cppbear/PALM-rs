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
impl fmt::Debug for Lcg128CmDxsm64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lcg128CmDxsm64 {{}}")
    }
}
