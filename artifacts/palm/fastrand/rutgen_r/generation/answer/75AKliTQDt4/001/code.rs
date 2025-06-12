// Answer 0

#[test]
fn test_mul_high_u128_zero() {
    assert_eq!(mul_high_u128(0, 0), 0);
}

#[test]
fn test_mul_high_u128_one() {
    assert_eq!(mul_high_u128(1, 1), 0);
}

#[test]
fn test_mul_high_u128_small_values() {
    assert_eq!(mul_high_u128(10, 20), 0);
}

#[test]
fn test_mul_high_u128_moderate_values() {
    assert_eq!(mul_high_u128(1000, 2000), 0);
}

#[test]
fn test_mul_high_u128_large_values() {
    assert_eq!(mul_high_u128(1 << 64, 1 << 64), 1);
}

#[test]
fn test_mul_high_u128_max_values() {
    assert_eq!(mul_high_u128(u128::MAX, u128::MAX), 1);
}

#[test]
fn test_mul_high_u128_mixed_values() {
    let a = (1 << 63) + 1;
    let b = (1 << 63) + 1;
    assert_eq!(mul_high_u128(a, b), 1);
}

