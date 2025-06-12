// Answer 0

#[test]
fn test_lowest_set_bit_with_nonzero_bits() {
    // Create a BitMask where the lowest set bit is at index 0 (binary: 0001)
    let bitmask = BitMask(0b0001);
    assert_eq!(bitmask.lowest_set_bit(), Some(0));

    // Create a BitMask where the lowest set bit is at index 1 (binary: 0010)
    let bitmask = BitMask(0b0010);
    assert_eq!(bitmask.lowest_set_bit(), Some(1));

    // Create a BitMask where the lowest set bit is at index 3 (binary: 1000)
    let bitmask = BitMask(0b1000);
    assert_eq!(bitmask.lowest_set_bit(), Some(3));

    // Create a BitMask with multiple bits set, lowest set bit at index 2 (binary: 0101)
    let bitmask = BitMask(0b0101);
    assert_eq!(bitmask.lowest_set_bit(), Some(0));

    // Create a BitMask with multiple bits set, lowest set bit at index 5 (binary: 1100)
    let bitmask = BitMask(0b1100);
    assert_eq!(bitmask.lowest_set_bit(), Some(2));
}

#[test]
fn test_lowest_set_bit_with_zero() {
    // Create a BitMask with no bits set (binary: 0000)
    let bitmask = BitMask(0);
    assert_eq!(bitmask.lowest_set_bit(), None);
}

