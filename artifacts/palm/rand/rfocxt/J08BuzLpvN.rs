use core::mem::size_of_val;
use rand_core::block::{BlockRng, BlockRngCore, CryptoBlockRng};
use rand_core::{CryptoRng, RngCore, SeedableRng, TryCryptoRng, TryRngCore};
#[derive(Debug)]
struct ReseedingCore<R, Rsdr> {
    inner: R,
    reseeder: Rsdr,
    threshold: i64,
    bytes_until_reseed: i64,
}
impl<R, Rsdr> ReseedingCore<R, Rsdr>
where
    R: BlockRngCore + SeedableRng,
    Rsdr: TryRngCore,
{
    fn new(threshold: u64, mut reseeder: Rsdr) -> Result<Self, Rsdr::Error> {
        let threshold = if threshold == 0 {
            i64::MAX
        } else if threshold <= i64::MAX as u64 {
            threshold as i64
        } else {
            i64::MAX
        };
        let inner = R::try_from_rng(&mut reseeder)?;
        Ok(ReseedingCore {
            inner,
            reseeder,
            threshold,
            bytes_until_reseed: threshold,
        })
    }
    fn reseed(&mut self) -> Result<(), Rsdr::Error> {}
    #[inline(never)]
    fn reseed_and_generate(&mut self, results: &mut <Self as BlockRngCore>::Results) {}
}
