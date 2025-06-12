use super::imp::{
    BitMaskWord, NonZeroBitMaskWord, BITMASK_ITER_MASK, BITMASK_MASK, BITMASK_STRIDE,
};
#[derive(Copy, Clone)]
pub(crate) struct BitMask(pub(crate) BitMaskWord);
#[allow(clippy::use_self)]
impl BitMask {
    #[inline]
    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn invert(self) -> Self {
        BitMask(self.0 ^ BITMASK_MASK)
    }
    #[inline]
    #[must_use]
    fn remove_lowest_bit(self) -> Self {
        BitMask(self.0 & (self.0 - 1))
    }
    #[inline]
    pub(crate) fn any_bit_set(self) -> bool {}
    #[inline]
    pub(crate) fn lowest_set_bit(self) -> Option<usize> {
        if let Some(nonzero) = NonZeroBitMaskWord::new(self.0) {
            Some(Self::nonzero_trailing_zeros(nonzero))
        } else {
            None
        }
    }
    #[inline]
    pub(crate) fn trailing_zeros(self) -> usize {}
    #[inline]
    fn nonzero_trailing_zeros(nonzero: NonZeroBitMaskWord) -> usize {
        if cfg!(target_arch = "arm") && BITMASK_STRIDE % 8 == 0 {
            let swapped = unsafe {
                NonZeroBitMaskWord::new_unchecked(nonzero.get().swap_bytes())
            };
            swapped.leading_zeros() as usize / BITMASK_STRIDE
        } else {
            nonzero.trailing_zeros() as usize / BITMASK_STRIDE
        }
    }
    #[inline]
    pub(crate) fn leading_zeros(self) -> usize {}
}
