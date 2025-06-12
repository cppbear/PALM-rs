use crate::{Rng, RngCore};
pub(crate) struct IncreasingUniform<R: RngCore> {
    pub rng: R,
    n: u32,
    chunk: u32,
    chunk_remaining: u8,
}
impl<R: RngCore> IncreasingUniform<R> {
    pub fn new(rng: R, n: u32) -> Self {
        let chunk_remaining = if n == 0 { 1 } else { 0 };
        Self {
            rng,
            n,
            chunk: 0,
            chunk_remaining,
        }
    }
    #[inline]
    pub fn next_index(&mut self) -> usize {}
}
