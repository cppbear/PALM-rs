// Answer 0

#[test]
fn test_any_bit_set_with_set_bit() {
    let bitmask = BitMask(0b0000_0001); // at least one bit is set
    assert!(bitmask.any_bit_set());
}

#[test]
fn test_any_bit_set_without_set_bit() {
    let bitmask = BitMask(0b0000_0000); // no bits are set
    assert!(!bitmask.any_bit_set());
}

#[test]
fn test_any_bit_set_with_multiple_set_bits() {
    let bitmask = BitMask(0b1010_1010); // multiple bits are set
    assert!(bitmask.any_bit_set());
}

#[test]
fn test_any_bit_set_with_highest_bit_set() {
    let bitmask = BitMask(0b1000_0000); // only the highest bit is set
    assert!(bitmask.any_bit_set());
}

#[test]
fn test_any_bit_set_with_all_bits_set() {
    let bitmask = BitMask(0b1111_1111); // all bits are set
    assert!(bitmask.any_bit_set());
}

