// Answer 0

#[test]
fn test_leading_zeros_all_ones() {
    let bitmask = BitMask(!0); // Assuming BITMASK_MASK is all bits set.
    assert_eq!(bitmask.leading_zeros(), 0); // No leading zeros
}

#[test]
fn test_leading_zeros_no_bits_set() {
    let bitmask = BitMask(0); // All bits clear
    assert_eq!(bitmask.leading_zeros(), BITMASK_WORD_SIZE / BITMASK_STRIDE); // All leading bits are zero
}

#[test]
fn test_leading_zeros_some_bits_set() {
    let bitmask = BitMask(0b0000_0000_0000_1111); // Example with some bits set
    assert_eq!(bitmask.leading_zeros(), 12 / BITMASK_STRIDE); // 12 leading zeros
}

#[test]
fn test_leading_zeros_edge_case() {
    let bitmask = BitMask(0b0000_0001); // Edge case with minimal set
    assert_eq!(bitmask.leading_zeros(), 11 / BITMASK_STRIDE); // 11 leading zeros
}

