// Answer 0

#[test]
fn test_nonzero_trailing_zeros_least_significant_bit() {
    let value = NonZeroBitMaskWord::new_unchecked(1);
    nonzero_trailing_zeros(value);
}

#[test]
fn test_nonzero_trailing_zeros_highest_significant_bit() {
    let value = NonZeroBitMaskWord::new_unchecked(0b10000000);
    nonzero_trailing_zeros(value);
}

#[test]
fn test_nonzero_trailing_zeros_all_bits_set() {
    let value = NonZeroBitMaskWord::new_unchecked(u64::MAX);
    nonzero_trailing_zeros(value);
}

#[test]
fn test_nonzero_trailing_zeros_random_pattern() {
    let value = NonZeroBitMaskWord::new_unchecked(0b101010);
    nonzero_trailing_zeros(value);
}

#[test]
fn test_nonzero_trailing_zeros_large_value() {
    let value = NonZeroBitMaskWord::new_unchecked(0b1000000000000000000000000000000000000000000000000000000000000001);
    nonzero_trailing_zeros(value);
}

#[test]
fn test_nonzero_trailing_zeros_various_single_bits() {
    let values = [
        NonZeroBitMaskWord::new_unchecked(1),
        NonZeroBitMaskWord::new_unchecked(2),
        NonZeroBitMaskWord::new_unchecked(4),
        NonZeroBitMaskWord::new_unchecked(8),
        NonZeroBitMaskWord::new_unchecked(16),
        NonZeroBitMaskWord::new_unchecked(32),
        NonZeroBitMaskWord::new_unchecked(64),
    ];
    for &value in &values {
        nonzero_trailing_zeros(value);
    }
}

