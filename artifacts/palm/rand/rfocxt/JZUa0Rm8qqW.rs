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
impl<R, Rsdr> BlockRngCore for ReseedingCore<R, Rsdr>
where
    R: BlockRngCore + SeedableRng,
    Rsdr: TryRngCore,
{
    type Item = <R as BlockRngCore>::Item;
    type Results = <R as BlockRngCore>::Results;
    fn generate(&mut self, results: &mut Self::Results) {
        if self.bytes_until_reseed <= 0 {
            return self.reseed_and_generate(results);
        }
        let num_bytes = size_of_val(results.as_ref());
        self.bytes_until_reseed -= num_bytes as i64;
        self.inner.generate(results);
    }
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
    fn reseed_and_generate(&mut self, results: &mut <Self as BlockRngCore>::Results) {
        trace!("Reseeding RNG (periodic reseed)");
        let num_bytes = size_of_val(results.as_ref());
        if let Err(e) = self.reseed() {
            warn!("Reseeding RNG failed: {}", e);
            let _ = e;
        }
        self.bytes_until_reseed = self.threshold - num_bytes as i64;
        self.inner.generate(results);
    }
}
