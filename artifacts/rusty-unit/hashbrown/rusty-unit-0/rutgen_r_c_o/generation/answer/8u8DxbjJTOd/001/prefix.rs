// Answer 0

#[test]
fn test_remove_lowest_bit_zero() {
    let bitmask = BitMask(0);
    let result = bitmask.remove_lowest_bit();
}

#[test]
fn test_remove_lowest_bit_one() {
    let bitmask = BitMask(1);
    let result = bitmask.remove_lowest_bit();
}

#[test]
fn test_remove_lowest_bit_two() {
    let bitmask = BitMask(2);
    let result = bitmask.remove_lowest_bit();
}

#[test]
fn test_remove_lowest_bit_three() {
    let bitmask = BitMask(3);
    let result = bitmask.remove_lowest_bit();
}

#[test]
fn test_remove_lowest_bit_four() {
    let bitmask = BitMask(4);
    let result = bitmask.remove_lowest_bit();
}

#[test]
fn test_remove_lowest_bit_large_value() {
    let bitmask = BitMask(123456789);
    let result = bitmask.remove_lowest_bit();
}

#[test]
fn test_remove_lowest_bit_max() {
    let bitmask = BitMask(u64::MAX);
    let result = bitmask.remove_lowest_bit();
}

