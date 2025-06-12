use core::mem::size_of_val;
use rand_core::block::{BlockRng, BlockRngCore, CryptoBlockRng};
use rand_core::{CryptoRng, RngCore, SeedableRng, TryCryptoRng, TryRngCore};
#[derive(Debug)]
pub struct ReseedingRng<R, Rsdr>(
    BlockRng<ReseedingCore<R, Rsdr>>,
)
where
    R: BlockRngCore + SeedableRng,
    Rsdr: TryRngCore;
#[derive(Debug)]
struct ReseedingCore<R, Rsdr> {
    inner: R,
    reseeder: Rsdr,
    threshold: i64,
    bytes_until_reseed: i64,
}
impl<R, Rsdr> RngCore for ReseedingRng<R, Rsdr>
where
    R: BlockRngCore<Item = u32> + SeedableRng,
    Rsdr: TryRngCore,
{
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {}
    fn fill_bytes(&mut self, dest: &mut [u8]) {}
}
