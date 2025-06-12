// Answer 0

#[test]
fn test_invert_all_bits_set() {
    let bitmask = BitMask(BITMASK_MASK);
    let inverted = bitmask.invert();
    assert_eq!(inverted.0, 0);
}

#[test]
fn test_invert_no_bits_set() {
    let bitmask = BitMask(0);
    let inverted = bitmask.invert();
    assert_eq!(inverted.0, BITMASK_MASK);
}

#[test]
fn test_invert_some_bits_set() {
    let bitmask = BitMask(0b10101010);
    let inverted = bitmask.invert();
    assert_eq!(inverted.0, !0b10101010 & BITMASK_MASK);
}

