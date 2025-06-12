pub type Pcg64Dxsm = Lcg128CmDxsm64;
use core::fmt;
use rand_core::{impls, le, RngCore, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
const MULTIPLIER: u64 = 15750249268501108917;
#[inline(always)]
fn output_dxsm(state: u128) -> u64 {
    let mut hi = (state >> 64) as u64;
    let mut lo = state as u64;
    lo |= 1;
    hi ^= hi >> 32;
    hi = hi.wrapping_mul(MULTIPLIER);
    hi ^= hi >> 48;
    hi = hi.wrapping_mul(lo);
    hi
}
