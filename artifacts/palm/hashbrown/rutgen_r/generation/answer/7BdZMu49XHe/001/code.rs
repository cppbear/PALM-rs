// Answer 0

#[test]
fn test_lowest_set_bit_none() {
    struct BitMask(u64);

    impl BitMask {
        pub(crate) fn lowest_set_bit(self) -> Option<usize> {
            if let Some(nonzero) = NonZeroBitMaskWord::new(self.0) {
                Some(Self::nonzero_trailing_zeros(nonzero))
            } else {
                None
            }
        }
    }

    struct NonZeroBitMaskWord(u64);

    impl NonZeroBitMaskWord {
        pub fn new(value: u64) -> Option<Self> {
            if value != 0 {
                Some(NonZeroBitMaskWord(value))
            } else {
                None
            }
        }
    }

    impl BitMask {
        pub fn nonzero_trailing_zeros(nonzero: NonZeroBitMaskWord) -> usize {
            let value = nonzero.0;
            value.trailing_zeros() as usize
        }
    }

    // Test case where the BitMask has no bits set (value of 0)
    let bitmask_zero = BitMask(0);
    assert_eq!(bitmask_zero.lowest_set_bit(), None);
}

