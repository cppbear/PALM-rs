// Answer 0

#[test]
fn test_any_bit_set_with_set_bits() {
    struct BitMask(u32);
    
    let bitmask = BitMask(0b00000001); // One bit set
    assert!(bitmask.any_bit_set());

    let bitmask = BitMask(0b00001111); // Multiple bits set
    assert!(bitmask.any_bit_set());

    let bitmask = BitMask(0b11111111); // All bits set
    assert!(bitmask.any_bit_set());
}

#[test]
fn test_any_bit_set_with_no_set_bits() {
    struct BitMask(u32);

    let bitmask = BitMask(0b00000000); // No bits set
    assert!(!bitmask.any_bit_set());
}

