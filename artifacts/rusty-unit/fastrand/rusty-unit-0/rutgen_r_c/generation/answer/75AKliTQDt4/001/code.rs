// Answer 0

#[test]
fn test_mul_high_u128_small_numbers() {
    let a: u128 = 2;
    let b: u128 = 3;
    let result = mul_high_u128(a, b);
    assert_eq!(result, 0);
}

#[test]
fn test_mul_high_u128_larger_numbers() {
    let a: u128 = 0x0000000100000000; // 2^32
    let b: u128 = 0x0000000100000000; // 2^32
    let result = mul_high_u128(a, b);
    assert_eq!(result, 1); // (2^32 * 2^32) >> 128 = 1
}

#[test]
fn test_mul_high_u128_max_values() {
    let a: u128 = u128::MAX; // 2^128 - 1
    let b: u128 = u128::MAX; // 2^128 - 1
    let result = mul_high_u128(a, b);
    assert_eq!(result, 3); // ((2^128 - 1) * (2^128 - 1)) >> 128 = (2^256 - 2^129 + 1) >> 128 = 3
}

#[test]
fn test_mul_high_u128_zero_multiplicand() {
    let a: u128 = 0;
    let b: u128 = 5;
    let result = mul_high_u128(a, b);
    assert_eq!(result, 0); // any number times zero is zero
}

#[test]
fn test_mul_high_u128_high_value() {
    let a: u128 = 0x0000FFFF00000000; // 2^64
    let b: u128 = 0x0000FFFF00000000; // 2^64
    let result = mul_high_u128(a, b);
    assert_eq!(result, 1); // (2^64 * 2^64) >> 128 = 1
}

#[test]
fn test_mul_high_u128_mixed_values() {
    let a: u128 = 0x0000000000000001; // 1
    let b: u128 = 0xFFFFFFFFFFFFFFFF; // max u64
    let result = mul_high_u128(a, b);
    assert_eq!(result, 0); // (1 * (2^64 - 1)) >> 128 = 0
}

