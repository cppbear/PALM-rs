// Answer 0

#[test]
fn test_remove_lowest_bit_zero() {
    let bitmask = BitMask(0);
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result, BitMask(0));
}

#[test]
fn test_remove_lowest_bit_single_bit_set() {
    let bitmask = BitMask(1); // 0b0001
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result, BitMask(0)); // 0b0000
}

#[test]
fn test_remove_lowest_bit_multiple_bits_set() {
    let bitmask = BitMask(0b0110); // multiple bits set: 0b0110
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result, BitMask(0b0100)); // 0b0100
}

#[test]
fn test_remove_lowest_bit_all_bits_set() {
    let bitmask = BitMask(!0); // all bits set
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result, BitMask(!0 & (!0 - 1))); // expecting one less than all bits set
}

