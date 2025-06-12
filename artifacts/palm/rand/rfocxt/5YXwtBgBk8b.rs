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
    fn flip_c_heads(&mut self, mut c: u32) -> bool {}
}
