// Answer 0

#[test]
fn test_invert_all_bits_set() {
    let bitmask = BitMask(!0); // All bits set
    let inverted = bitmask.invert();
    assert_eq!(inverted.0, !0 ^ BITMASK_MASK);
}

#[test]
fn test_invert_no_bits_set() {
    let bitmask = BitMask(0); // No bits set
    let inverted = bitmask.invert();
    assert_eq!(inverted.0, 0 ^ BITMASK_MASK);
}

#[test]
fn test_invert_some_bits_set() {
    let bitmask = BitMask(0b10101010); // Some bits set
    let inverted = bitmask.invert();
    assert_eq!(inverted.0, 0b10101010 ^ BITMASK_MASK);
}

#[test]
fn test_invert_edge_case() {
    let bitmask = BitMask(BITMASK_MASK); // Edge case where we have a specific mask
    let inverted = bitmask.invert();
    assert_eq!(inverted.0, BITMASK_MASK ^ BITMASK_MASK); // Should result in 0
}

