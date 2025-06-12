// Answer 0

#[test]
fn test_remove_lowest_bit_zero() {
    let bitmask = BitMask(0);
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result.0, 0);
}

#[test]
fn test_remove_lowest_bit_one() {
    let bitmask = BitMask(1);
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result.0, 0);
}

#[test]
fn test_remove_lowest_bit_two() {
    let bitmask = BitMask(2);
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result.0, 0);
}

#[test]
fn test_remove_lowest_bit_three() {
    let bitmask = BitMask(3);
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result.0, 2);
}

#[test]
fn test_remove_lowest_bit_five() {
    let bitmask = BitMask(5);
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result.0, 4);
}

#[test]
fn test_remove_lowest_bit_multiple_bits() {
    let bitmask = BitMask(14); // 1110 in binary
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result.0, 12); // 1100 in binary
}

