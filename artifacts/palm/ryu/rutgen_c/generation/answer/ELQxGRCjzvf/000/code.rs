// Answer 0

#[test]
fn test_d2d_zero() {
    let result = d2d(0, 0);
    assert_eq!(result.mantissa, 0);
    assert_eq!(result.exponent, 1 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2);
}

#[test]
fn test_d2d_small_mantissa() {
    let result = d2d(1, 1);
    assert_eq!(result.mantissa, 4);
    assert_eq!(result.exponent, -1022);
}

#[test]
fn test_d2d_even_mantissa() {
    let result = d2d(2, 3);
    assert_eq!(result.mantissa, 9);
    assert_eq!(result.exponent, -1021);
}

#[test]
fn test_d2d_odd_mantissa() {
    let result = d2d(3, 4);
    let expected_exponent = log10_pow2(-1020); // Using general logging
    assert_eq!(result.mantissa, 12);
    assert_eq!(result.exponent, expected_exponent);
}

#[test]
fn test_d2d_large_exponent() {
    let result = d2d(0xFFFFFFFFFFFFFFFF, 1111);
    assert!(result.mantissa > 0);
    assert_eq!(result.exponent, -1023); // As per the exponent handling
}

#[test]
fn test_d2d_small_exponent() {
    let result = d2d(0x1, 0);
    assert_eq!(result.mantissa, 4);
    assert_eq!(result.exponent, 1 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2);
}

