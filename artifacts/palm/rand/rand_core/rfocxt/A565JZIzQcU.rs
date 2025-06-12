use crate::impls::fill_via_chunks;
use crate::{CryptoRng, RngCore, SeedableRng, TryRngCore};
use core::fmt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
pub trait SeedableRng: Sized {
    type Seed: Clone + Default + AsRef<[u8]> + AsMut<[u8]>;
    fn from_seed(seed: Self::Seed) -> Self;
    fn seed_from_u64(mut state: u64) -> Self {
        fn pcg32(state: &mut u64) -> [u8; 4] {
            const MUL: u64 = 6364136223846793005;
            const INC: u64 = 11634580027462260723;
            *state = state.wrapping_mul(MUL).wrapping_add(INC);
            let state = *state;
            let xorshifted = (((state >> 18) ^ state) >> 27) as u32;
            let rot = (state >> 59) as u32;
            let x = xorshifted.rotate_right(rot);
            x.to_le_bytes()
        }
        let mut seed = Self::Seed::default();
        let mut iter = seed.as_mut().chunks_exact_mut(4);
        for chunk in &mut iter {
            chunk.copy_from_slice(&pcg32(&mut state));
        }
        let rem = iter.into_remainder();
        if !rem.is_empty() {
            rem.copy_from_slice(&pcg32(&mut state)[..rem.len()]);
        }
        Self::from_seed(seed)
    }
    fn from_rng(rng: &mut impl RngCore) -> Self {
        let mut seed = Self::Seed::default();
        rng.fill_bytes(seed.as_mut());
        Self::from_seed(seed)
    }
    fn try_from_rng<R: TryRngCore>(rng: &mut R) -> Result<Self, R::Error> {
        let mut seed = Self::Seed::default();
        rng.try_fill_bytes(seed.as_mut())?;
        Ok(Self::from_seed(seed))
    }
    #[cfg(feature = "os_rng")]
    fn from_os_rng() -> Self {
        match Self::try_from_os_rng() {
            Ok(res) => res,
            Err(err) => panic!("from_os_rng failed: {}", err),
        }
    }
    #[cfg(feature = "os_rng")]
    fn try_from_os_rng() -> Result<Self, getrandom::Error> {
        let mut seed = Self::Seed::default();
        getrandom::fill(seed.as_mut())?;
        let res = Self::from_seed(seed);
        Ok(res)
    }
}
pub trait RngCore {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;
    fn fill_bytes(&mut self, dst: &mut [u8]);
}
pub trait BlockRngCore {
    type Item;
    type Results: AsRef<[Self::Item]> + AsMut<[Self::Item]> + Default;
    fn generate(&mut self, results: &mut Self::Results);
}
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BlockRng64<R: BlockRngCore + ?Sized> {
    results: R::Results,
    index: usize,
    half_used: bool,
    /// The *core* part of the RNG, implementing the `generate` function.
    pub core: R,
}
impl<R: BlockRngCore> BlockRng64<R> {
    #[inline]
    pub fn new(core: R) -> BlockRng64<R> {}
    #[inline(always)]
    pub fn index(&self) -> usize {
        self.index
    }
    #[inline]
    pub fn reset(&mut self) {}
    #[inline]
    pub fn generate_and_set(&mut self, index: usize) {}
}
