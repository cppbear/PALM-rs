use crate::RngCore;
pub(crate) struct CoinFlipper<R: RngCore> {
    pub rng: R,
    chunk: u32,
    chunk_remaining: u32,
}
impl<R: RngCore> CoinFlipper<R> {
    pub fn new(rng: R) -> Self {
        Self {
            rng,
            chunk: 0,
            chunk_remaining: 0,
        }
    }
    #[inline]
    pub fn random_ratio_one_over(&mut self, d: usize) -> bool {}
    #[inline]
    fn random_ratio(&mut self, mut n: usize, d: usize) -> bool {}
    fn flip_c_heads(&mut self, mut c: u32) -> bool {
        debug_assert!(c <= 32);
        loop {
            let zeros = self.chunk.leading_zeros();
            if zeros < c {
                self.chunk = self.chunk.wrapping_shl(zeros + 1);
                self.chunk_remaining = self.chunk_remaining.saturating_sub(zeros + 1);
                return false;
            } else {
                if let Some(new_remaining) = self.chunk_remaining.checked_sub(c) {
                    self.chunk_remaining = new_remaining;
                    self.chunk <<= c;
                    return true;
                } else {
                    c -= self.chunk_remaining;
                    self.chunk = self.rng.next_u32();
                    self.chunk_remaining = 32;
                }
            }
        }
    }
}
