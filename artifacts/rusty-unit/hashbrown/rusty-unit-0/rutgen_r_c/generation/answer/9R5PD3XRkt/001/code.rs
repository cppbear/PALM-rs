// Answer 0

#[test]
fn test_leading_zeros_all_bits_set() {
    let bitmask = BitMask(!0); // All bits set (assuming BitMaskWord is a type that can represent this)
    assert_eq!(bitmask.leading_zeros(), 0); // No leading zeros
}

#[test]
fn test_leading_zeros_some_bits_set() {
    let bitmask = BitMask(0b00001111); // Leading zeros exist
    assert_eq!(bitmask.leading_zeros(), 4); // Four leading zeros
}

#[test]
fn test_leading_zeros_one_bit_set() {
    let bitmask = BitMask(0b00000001); // Only the lowest bit set
    assert_eq!(bitmask.leading_zeros(), 7); // Seven leading zeros (assuming 8-bit representation)
}

#[test]
fn test_leading_zeros_empty_bitmask() {
    let bitmask = BitMask(0); // No bits set
    assert_eq!(bitmask.leading_zeros(), 8); // Again, depending on the assumed bit representation size
}

