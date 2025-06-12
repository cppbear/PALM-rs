// Answer 0

#[test]
fn test_lowest_set_bit_some() {
    // Assuming a suitable BitMaskWord value that sets the lowest bit
    let bitmask_value = 0b0000_0001; // 1 in binary
    let bitmask = BitMask(bitmask_value);
    let result = bitmask.lowest_set_bit();
    assert_eq!(result, Some(0)); // The index of the lowest set bit is 0
}

#[test]
fn test_lowest_set_bit_none() {
    // Case where no bits are set
    let bitmask_value = 0b0000_0000; // 0 in binary
    let bitmask = BitMask(bitmask_value);
    let result = bitmask.lowest_set_bit();
    assert_eq!(result, None); // No bits are set, should return None
}

#[test]
fn test_lowest_set_bit_multiple_bits() {
    // Case where multiple bits are set
    let bitmask_value = 0b0000_1001; // Only lowest set bit is at index 0
    let bitmask = BitMask(bitmask_value);
    let result = bitmask.lowest_set_bit();
    assert_eq!(result, Some(0)); // The index of the lowest set bit is still 0
}

#[test]
fn test_lowest_set_bit_high_bit() {
    // Case with only the highest bit set
    let bitmask_value = 0b1000_0000; // Highest bit is at index 7
    let bitmask = BitMask(bitmask_value);
    let result = bitmask.lowest_set_bit();
    assert_eq!(result, Some(7)); // The index of the lowest set bit is 7
}

#[test]
fn test_lowest_set_bit_complex() {
    // Complex case with bits set in a non-standard pattern
    let bitmask_value = 0b0100_1000; // Set bits at indices 3 and 6
    let bitmask = BitMask(bitmask_value);
    let result = bitmask.lowest_set_bit();
    assert_eq!(result, Some(3)); // The index of the lowest set bit is 3
}

