// Answer 0

#[test]
fn test_nonzero_trailing_zeros_arm() {
    struct NonZeroBitMaskWord(usize);

    impl NonZeroBitMaskWord {
        fn new_unchecked(value: usize) -> Self {
            NonZeroBitMaskWord(value)
        }

        fn get(&self) -> usize {
            self.0
        }

        fn trailing_zeros(&self) -> u32 {
            self.0.trailing_zeros()
        }

        fn leading_zeros(&self) -> u32 {
            self.0.leading_zeros()
        }
    }

    const BITMASK_STRIDE: usize = 8;

    // Test cases (assuming arm architecture and BITMASK_STRIDE is 8)
    let nonzero = NonZeroBitMaskWord::new_unchecked(0b0000_0001);
    assert_eq!(nonzero_trailing_zeros(nonzero), 1 / BITMASK_STRIDE);

    let nonzero = NonZeroBitMaskWord::new_unchecked(0b0000_0010);
    assert_eq!(nonzero_trailing_zeros(nonzero), 2 / BITMASK_STRIDE);

    let nonzero = NonZeroBitMaskWord::new_unchecked(0b0000_1111);
    assert_eq!(nonzero_trailing_zeros(nonzero), 0 / BITMASK_STRIDE);

    let nonzero = NonZeroBitMaskWord::new_unchecked(0b1111_1110);
    assert_eq!(nonzero_trailing_zeros(nonzero), 1 / BITMASK_STRIDE);

    let nonzero = NonZeroBitMaskWord::new_unchecked(0b0000_0000_0000_0001);
    assert_eq!(nonzero_trailing_zeros(nonzero), 1 / BITMASK_STRIDE);
}

#[test]
fn test_nonzero_trailing_zeros_generic() {
    struct NonZeroBitMaskWord(usize);

    impl NonZeroBitMaskWord {
        fn new_unchecked(value: usize) -> Self {
            NonZeroBitMaskWord(value)
        }

        fn get(&self) -> usize {
            self.0
        }

        fn trailing_zeros(&self) -> u32 {
            self.0.trailing_zeros()
        }

        // We define leading_zeros only to cater to the first test case, thus not overriding behavior.
        fn leading_zeros(&self) -> u32 {
            self.0.leading_zeros()
        }
    }

    const BITMASK_STRIDE: usize = 16;

    // Check case for leading zeros swap under generic BITMASK_STRIDE
    let nonzero = NonZeroBitMaskWord::new_unchecked(0b10000000000000000);
    assert_eq!(nonzero_trailing_zeros(nonzero), 15 / BITMASK_STRIDE);
}

