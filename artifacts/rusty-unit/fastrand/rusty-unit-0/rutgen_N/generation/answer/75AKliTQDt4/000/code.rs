// Answer 0

#[test]
fn test_mul_high_u128_small_values() {
    assert_eq!(mul_high_u128(1, 1), 0);
    assert_eq!(mul_high_u128(1, 2), 0);
    assert_eq!(mul_high_u128(2, 2), 0);
}

#[test]
fn test_mul_high_u128_boundary_values() {
    assert_eq!(mul_high_u128(0, 0), 0);
    assert_eq!(mul_high_u128(u128::MAX, 1), 0);
    assert_eq!(mul_high_u128(1, u128::MAX), 0);
}

#[test]
fn test_mul_high_u128_large_values() {
    assert_eq!(mul_high_u128(u128::MAX, u128::MAX), u128::MAX >> 1);
    assert_eq!(mul_high_u128(1 << 128, 1 << 128), 1);
}

