// Answer 0

#[test]
fn test_leading_zeros_all_set() {
    let bitmask = BitMask(0xFFFFFFFFFFFFFFFF);
    let result = bitmask.leading_zeros();
}

#[test]
fn test_leading_zeros_none_set() {
    let bitmask = BitMask(0);
    let result = bitmask.leading_zeros();
}

#[test]
fn test_leading_zeros_only_first_bit_set() {
    let bitmask = BitMask(0x0000000000000001);
    let result = bitmask.leading_zeros();
}

#[test]
fn test_leading_zeros_only_second_bit_set() {
    let bitmask = BitMask(0x0000000000000002);
    let result = bitmask.leading_zeros();
}

#[test]
fn test_leading_zeros_random_middle_bit_set() {
    let bitmask = BitMask(0x0000000000000800);
    let result = bitmask.leading_zeros();
}

#[test]
fn test_leading_zeros_alternating_bits_set() {
    let bitmask = BitMask(0xAAAAAAAAAAAAAAAA);
    let result = bitmask.leading_zeros();
}

#[test]
fn test_leading_zeros_first_half_bits_set() {
    let bitmask = BitMask(0xFFFFFFFF00000000);
    let result = bitmask.leading_zeros();
}

#[test]
fn test_leading_zeros_last_half_bits_set() {
    let bitmask = BitMask(0x00000000FFFFFFFF);
    let result = bitmask.leading_zeros();
}

