// Answer 0

#[test]
fn test_trailing_zeros_zero() {
    struct BitMask(u64);
    const BITMASK_STRIDE: usize = 8; // Assuming a value as it's not defined

    let mask = BitMask(0);
    let result = mask.trailing_zeros();
    assert_eq!(result, 0);
}

#[test]
fn test_trailing_zeros_single_bit() {
    struct BitMask(u64);
    const BITMASK_STRIDE: usize = 8;

    let mask = BitMask(1);
    let result = mask.trailing_zeros();
    assert_eq!(result, 0);
}

#[test]
fn test_trailing_zeros_two_bits() {
    struct BitMask(u64);
    const BITMASK_STRIDE: usize = 8;

    let mask = BitMask(2);
    let result = mask.trailing_zeros();
    assert_eq!(result, 1);
}

#[test]
fn test_trailing_zeros_four_bits() {
    struct BitMask(u64);
    const BITMASK_STRIDE: usize = 8;

    let mask = BitMask(8);
    let result = mask.trailing_zeros();
    assert_eq!(result, 3);
}

#[test]
fn test_trailing_zeros_multiple_bits() {
    struct BitMask(u64);
    const BITMASK_STRIDE: usize = 8;

    let mask = BitMask(16);
    let result = mask.trailing_zeros();
    assert_eq!(result, 4);
}

#[test]
fn test_trailing_zeros_full_byte() {
    struct BitMask(u64);
    const BITMASK_STRIDE: usize = 8;

    let mask = BitMask(255);
    let result = mask.trailing_zeros();
    assert_eq!(result, 0);
}

