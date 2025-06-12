use crate::impls::fill_via_chunks;
use crate::{CryptoRng, RngCore, SeedableRng, TryRngCore};
use core::fmt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
pub trait RngCore {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;
    fn fill_bytes(&mut self, dst: &mut [u8]);
}
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
pub trait BlockRngCore {
    type Item;
    type Results: AsRef<[Self::Item]> + AsMut<[Self::Item]> + Default;
    fn generate(&mut self, results: &mut Self::Results);
}
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        bound = "for<'x> R: Serialize + Deserialize<'x>, for<'x> R::Results: Serialize + Deserialize<'x>"
    )
)]
pub struct BlockRng<R: BlockRngCore> {
    results: R::Results,
    index: usize,
    /// The *core* part of the RNG, implementing the `generate` function.
    pub core: R,
}
impl<R: BlockRngCore<Item = u32>> RngCore for BlockRng<R> {
    #[inline]
    fn next_u32(&mut self) -> u32 {}
    #[inline]
    fn next_u64(&mut self) -> u64 {}
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut read_len = 0;
        while read_len < dest.len() {
            if self.index >= self.results.as_ref().len() {
                self.generate_and_set(0);
            }
            let (consumed_u32, filled_u8) = fill_via_chunks(
                &self.results.as_mut()[self.index..],
                &mut dest[read_len..],
            );
            self.index += consumed_u32;
            read_len += filled_u8;
        }
    }
}
impl<R: BlockRngCore> BlockRng<R> {
    #[inline]
    pub fn new(core: R) -> BlockRng<R> {}
    #[inline(always)]
    pub fn index(&self) -> usize {}
    #[inline]
    pub fn reset(&mut self) {}
    #[inline]
    pub fn generate_and_set(&mut self, index: usize) {
        assert!(index < self.results.as_ref().len());
        self.core.generate(&mut self.results);
        self.index = index;
    }
}
pub(crate) fn fill_via_chunks<T: Observable>(
    src: &[T],
    dest: &mut [u8],
) -> (usize, usize) {
    let size = core::mem::size_of::<T>();
    let mut dest = dest.chunks_exact_mut(size);
    let mut src = src.iter();
    let zipped = dest.by_ref().zip(src.by_ref());
    let num_chunks = zipped.len();
    zipped.for_each(|(dest, src)| dest.copy_from_slice(src.to_le_bytes().as_ref()));
    let byte_len = num_chunks * size;
    if let Some(src) = src.next() {
        let dest = dest.into_remainder();
        let n = dest.len();
        if n > 0 {
            dest.copy_from_slice(&src.to_le_bytes().as_ref()[..n]);
            return (num_chunks + 1, byte_len + n);
        }
    }
    (num_chunks, byte_len)
}
