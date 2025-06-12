// Answer 0

#[test]
fn test_any_bit_set_with_set_bit() {
    let bitmask = BitMask(0b0000_0001); // BitMask with the lowest bit set
    assert!(bitmask.any_bit_set());
}

#[test]
fn test_any_bit_set_with_multiple_set_bits() {
    let bitmask = BitMask(0b1010_1010); // BitMask with multiple bits set
    assert!(bitmask.any_bit_set());
}

#[test]
fn test_any_bit_set_with_no_set_bits() {
    let bitmask = BitMask(0b0000_0000); // BitMask with no bits set
    assert!(!bitmask.any_bit_set());
}

