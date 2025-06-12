use super::group::{
    BitMaskWord, NonZeroBitMaskWord, BITMASK_ITER_MASK, BITMASK_MASK, BITMASK_STRIDE,
};

/// A bit mask which contains the result of a `Match` operation on a `Group` and
/// allows iterating through them.
///
/// The bit mask is arranged so that low-order bits represent lower memory
/// addresses for group match results.
///
/// For implementation reasons, the bits in the set may be sparsely packed with
/// groups of 8 bits representing one element. If any of these bits are non-zero
/// then this element is considered to true in the mask. If this is the
/// case, `BITMASK_STRIDE` will be 8 to indicate a divide-by-8 should be
/// performed on counts/indices to normalize this difference. `BITMASK_MASK` is
/// similarly a mask of all the actually-used bits.
///
/// To iterate over a bit mask, it must be converted to a form where only 1 bit
/// is set per element. This is done by applying `BITMASK_ITER_MASK` on the
/// mask bits.
#[derive(Copy, Clone)]
pub(crate) struct BitMask(pub(crate) BitMaskWord);

#[allow(clippy::use_self)]
impl BitMask {
    /// Returns a new `BitMask` with all bits inverted.
    #[inline]
    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn invert(self) -> Self {
        BitMask(self.0 ^ BITMASK_MASK)
    }

    /// Returns a new `BitMask` with the lowest bit removed.
    #[inline]
    #[must_use]
    fn remove_lowest_bit(self) -> Self {
        BitMask(self.0 & (self.0 - 1))
    }

    /// Returns whether the `BitMask` has at least one set bit.
    #[inline]
    pub(crate) fn any_bit_set(self) -> bool {
        self.0 != 0
    }

    /// Returns the first set bit in the `BitMask`, if there is one.
    #[inline]
    pub(crate) fn lowest_set_bit(self) -> Option<usize> {
        if let Some(nonzero) = NonZeroBitMaskWord::new(self.0) {
            Some(Self::nonzero_trailing_zeros(nonzero))
        } else {
            None
        }
    }

    /// Returns the number of trailing zeroes in the `BitMask`.
    #[inline]
    pub(crate) fn trailing_zeros(self) -> usize {
        // ARM doesn't have a trailing_zeroes instruction, and instead uses
        // reverse_bits (RBIT) + leading_zeroes (CLZ). However older ARM
        // versions (pre-ARMv7) don't have RBIT and need to emulate it
        // instead. Since we only have 1 bit set in each byte on ARM, we can
        // use swap_bytes (REV) + leading_zeroes instead.
        if cfg!(target_arch = "arm") && BITMASK_STRIDE % 8 == 0 {
            self.0.swap_bytes().leading_zeros() as usize / BITMASK_STRIDE
        } else {
            self.0.trailing_zeros() as usize / BITMASK_STRIDE
        }
    }

    /// Same as above but takes a `NonZeroBitMaskWord`.
    #[inline]
    fn nonzero_trailing_zeros(nonzero: NonZeroBitMaskWord) -> usize {
        if cfg!(target_arch = "arm") && BITMASK_STRIDE % 8 == 0 {
            // SAFETY: A byte-swapped non-zero value is still non-zero.
            let swapped = unsafe { NonZeroBitMaskWord::new_unchecked(nonzero.get().swap_bytes()) };
            swapped.leading_zeros() as usize / BITMASK_STRIDE
        } else {
            nonzero.trailing_zeros() as usize / BITMASK_STRIDE
        }
    }

    /// Returns the number of leading zeroes in the `BitMask`.
    #[inline]
    pub(crate) fn leading_zeros(self) -> usize {
        self.0.leading_zeros() as usize / BITMASK_STRIDE
    }
}

impl IntoIterator for BitMask {
    type Item = usize;
    type IntoIter = BitMaskIter;

    #[inline]
    fn into_iter(self) -> BitMaskIter {
        // A BitMask only requires each element (group of bits) to be non-zero.
        // However for iteration we need each element to only contain 1 bit.
        BitMaskIter(BitMask(self.0 & BITMASK_ITER_MASK))
    }
}

/// Iterator over the contents of a `BitMask`, returning the indices of set
/// bits.
#[derive(Clone)]
pub(crate) struct BitMaskIter(pub(crate) BitMask);

impl Iterator for BitMaskIter {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        let bit = self.0.lowest_set_bit()?;
        self.0 = self.0.remove_lowest_bit();
        Some(bit)
    }
}

#[cfg(test)]
mod tests_llm_16_11 {
    use super::*;

use crate::*;
    use crate::control::bitmask::{BitMask, BitMaskIter};

    #[test]
    fn test_into_iter_empty() {
        let bitmask = BitMask(0);
        let mut iter = bitmask.into_iter();
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_into_iter_single_bit() {
        let bitmask = BitMask(1); // Only the first bit is set
        let mut iter = bitmask.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_into_iter_multiple_bits() {
        let bitmask = BitMask(0b101); // Bits at positions 0 and 2 are set
        let mut iter = bitmask.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(2));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_into_iter_non_consecutive_bits() {
        let bitmask = BitMask(0b100010); // Bits at positions 1 and 5 are set
        let mut iter = bitmask.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(5));
        assert!(iter.next().is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_12 {
    use super::*;

use crate::*;
    use crate::control::bitmask::BitMask;

    #[test]
    fn test_next() {
        let mut bitmask = BitMask(0b10100); // Example bitmask with bits 2 and 4 set
        let mut iter = bitmask.into_iter();

        // First call to next should return the lowest set bit (2)
        assert_eq!(iter.next(), Some(2));
        // Next call should return the next lowest set bit (4)
        assert_eq!(iter.next(), Some(4));
        // After all set bits are iterated, next should return None
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_next_with_no_bits_set() {
        let mut bitmask = BitMask(0b00000); // Bitmask with no bits set
        let mut iter = bitmask.into_iter();

        // Should return None since there are no set bits
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_next_with_single_bit_set() {
        let mut bitmask = BitMask(0b00010); // Bitmask with a single bit set (bit 1)
        let mut iter = bitmask.into_iter();

        // First call to next should return the only set bit (1)
        assert_eq!(iter.next(), Some(1));
        // Next call should return None
        assert_eq!(iter.next(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_231 {
    use super::*;

use crate::*;
    use crate::control::bitmask::BitMask;

    #[test]
    fn test_lowest_set_bit_some() {
        let mask = BitMask(0b00000010);
        assert_eq!(mask.lowest_set_bit(), Some(1));
    }

    #[test]
    fn test_lowest_set_bit_some_multiple_bits() {
        let mask = BitMask(0b00001100);
        assert_eq!(mask.lowest_set_bit(), Some(2));
    }

    #[test]
    fn test_lowest_set_bit_none() {
        let mask = BitMask(0b00000000);
        assert_eq!(mask.lowest_set_bit(), None);
    }

    #[test]
    fn test_lowest_set_bit_some_first_bit() {
        let mask = BitMask(0b00000001);
        assert_eq!(mask.lowest_set_bit(), Some(0));
    }
}

#[cfg(test)]
mod tests_llm_16_232 {
    use super::*;

use crate::*;
    use crate::control::bitmask::{BitMask, NonZeroBitMaskWord};

    #[test]
    fn test_nonzero_trailing_zeros() {
        // BitMask stride assumption, adjust it based on actual crate definition
        const BITMASK_STRIDE: usize = 8; // Update this according to the actual stride
        let test_cases = vec![
            (NonZeroBitMaskWord::new(0b0000_0001).unwrap(), 0), // 1 -> 0 trailing zeros
            (NonZeroBitMaskWord::new(0b0000_0010).unwrap(), 1), // 2 -> 1 trailing zero
            (NonZeroBitMaskWord::new(0b0000_1100).unwrap(), 2), // 12 -> 2 trailing zeros
            (NonZeroBitMaskWord::new(0b0001_0000).unwrap(), 4), // 16 -> 4 trailing zeros
            (NonZeroBitMaskWord::new(0b0010_0000).unwrap(), 5), // 32 -> 5 trailing zeros
            (NonZeroBitMaskWord::new(0b1111_1110).unwrap(), 1), // 254 -> 1 trailing zero
            (NonZeroBitMaskWord::new(0b1000_0000).unwrap(), 7), // 128 -> 7 trailing zeros
        ];

        for (input, expected) in test_cases {
            assert_eq!(BitMask::nonzero_trailing_zeros(input), expected);
        }
    }
}

#[cfg(test)]
mod tests_llm_16_233 {
    use super::*;

use crate::*;
    use crate::control::bitmask::BitMask;

    #[test]
    fn test_remove_lowest_bit() {
        let mask1 = BitMask(0b1010); // Binary: 1010
        let mask2 = mask1.remove_lowest_bit();
        assert_eq!(mask2.0, 0b1000); // Should remove the lowest set bit (1 in this case)

        let mask3 = BitMask(0b0000); // Binary: 0000
        let mask4 = mask3.remove_lowest_bit();
        assert_eq!(mask4.0, 0b0000); // Should remain unchanged, no bits set

        let mask5 = BitMask(0b0001); // Binary: 0001
        let mask6 = mask5.remove_lowest_bit();
        assert_eq!(mask6.0, 0b0000); // Should remove the only set bit

        let mask7 = BitMask(0b1111); // Binary: 1111
        let mask8 = mask7.remove_lowest_bit();
        assert_eq!(mask8.0, 0b1110); // Should remove the lowest set bit
    }
}
