// Answer 0

#[test]
fn test_lowest_set_bit_with_zero() {
    let bitmask = BitMask(0);
    assert_eq!(bitmask.lowest_set_bit(), None);
}

#[test]
fn test_lowest_set_bit_with_non_zero_mask() {
    let bitmask = BitMask(1); // Only the lowest bit is set
    assert_eq!(bitmask.lowest_set_bit(), Some(0));
}

#[test]
fn test_lowest_set_bit_with_multiple_bits_set() {
    let bitmask = BitMask(0b110); // The first set bit is at index 1
    assert_eq!(bitmask.lowest_set_bit(), Some(1));
}

#[test]
fn test_lowest_set_bit_with_maximum_value() {
    let bitmask = BitMask(u64::MAX); // All bits set
    assert_eq!(bitmask.lowest_set_bit(), Some(0)); // Should still return the first set bit
}

