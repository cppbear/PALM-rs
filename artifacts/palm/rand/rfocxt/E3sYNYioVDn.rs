use rand_core::impls::fill_bytes_via_next;
use rand_core::le::read_u64_into;
use rand_core::{RngCore, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Xoshiro256PlusPlus {
    s: [u64; 4],
}
impl SeedableRng for Xoshiro256PlusPlus {
    type Seed = [u8; 32];
    #[inline]
    fn from_seed(seed: [u8; 32]) -> Xoshiro256PlusPlus {
        let mut state = [0; 4];
        read_u64_into(&seed, &mut state);
        if state.iter().all(|&x| x == 0) {
            return Self::seed_from_u64(0);
        }
        Xoshiro256PlusPlus { s: state }
    }
    #[inline]
    fn seed_from_u64(mut state: u64) -> Self {
        const PHI: u64 = 0x9e3779b97f4a7c15;
        let mut s = [0; 4];
        for i in s.iter_mut() {
            state = state.wrapping_add(PHI);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z = z ^ (z >> 31);
            *i = z;
        }
        debug_assert_ne!(s, [0; 4]);
        Xoshiro256PlusPlus { s }
    }
}
