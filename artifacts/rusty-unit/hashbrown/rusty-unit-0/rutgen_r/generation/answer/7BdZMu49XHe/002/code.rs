// Answer 0

#[test]
fn test_lowest_set_bit_with_nonzero_bitmask() {
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
            // implementation to calculate trailing zeros; assume it returns a usize
            nonzero.0.trailing_zeros() as usize
        }
    }

    struct NonZeroBitMaskWord(u64);

    impl NonZeroBitMaskWord {
        pub fn new(value: u64) -> Option<Self> {
            if value != 0 {
                Some(Self(value))
            } else {
                None
            }
        }
    }

    // Test case 1: Lowest set bit at position 0 (value = 1)
    let bitmask_1 = BitMask(1);
    assert_eq!(bitmask_1.lowest_set_bit(), Some(0));

    // Test case 2: Lowest set bit at position 1 (value = 2)
    let bitmask_2 = BitMask(2);
    assert_eq!(bitmask_2.lowest_set_bit(), Some(1));

    // Test case 3: Lowest set bit at position 3 (value = 8)
    let bitmask_3 = BitMask(8);
    assert_eq!(bitmask_3.lowest_set_bit(), Some(3));

    // Test case 4: Lowest set bit at position 5 (value = 32)
    let bitmask_4 = BitMask(32);
    assert_eq!(bitmask_4.lowest_set_bit(), Some(5));

    // Test case 5: Lowest set bit at position 63 (value = 0x8000000000000000)
    let bitmask_5 = BitMask(0x8000000000000000);
    assert_eq!(bitmask_5.lowest_set_bit(), Some(63));
}

#[test]
fn test_lowest_set_bit_with_zero_bitmask() {
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
        pub fn new(value: u64) -> Option<Self> {
            if value != 0 {
                Some(Self(value))
            } else {
                None
            }
        }
    }

    // Test case: Bitmask is zero
    let bitmask = BitMask(0);
    assert_eq!(bitmask.lowest_set_bit(), None);
}

