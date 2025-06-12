// Answer 0

#[test]
fn test_lowest_set_bit_some() {
    struct BitMask(u64);
    impl BitMask {
        pub(crate) fn lowest_set_bit(self) -> Option<usize> {
            if let Some(nonzero) = NonZeroBitMaskWord::new(self.0) {
                Some(Self::nonzero_trailing_zeros(nonzero))
            } else {
                None
            }
        }

        fn nonzero_trailing_zeros(nonzero: NonZeroBitMaskWord) -> usize {
            nonzero.0.trailing_zeros() as usize
        }
    }

    struct NonZeroBitMaskWord(u64);
    impl NonZeroBitMaskWord {
        fn new(value: u64) -> Option<Self> {
            if value != 0 {
                Some(Self(value))
            } else {
                None
            }
        }
    }

    let bitmask = BitMask(0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001);
    assert_eq!(bitmask.lowest_set_bit(), Some(0));
}

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

        fn nonzero_trailing_zeros(nonzero: NonZeroBitMaskWord) -> usize {
            nonzero.0.trailing_zeros() as usize
        }
    }

    struct NonZeroBitMaskWord(u64);
    impl NonZeroBitMaskWord {
        fn new(value: u64) -> Option<Self> {
            if value != 0 {
                Some(Self(value))
            } else {
                None
            }
        }
    }

    let bitmask = BitMask(0);
    assert_eq!(bitmask.lowest_set_bit(), None);
}

