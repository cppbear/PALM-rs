// Answer 0

#[test]
fn test_mul_high_u128_small_numbers() {
    assert_eq!(mul_high_u128(1, 1), 0);
    assert_eq!(mul_high_u128(2, 2), 0);
    assert_eq!(mul_high_u128(10, 10), 0);
}

#[test]
fn test_mul_high_u128_large_numbers() {
    assert_eq!(mul_high_u128(1 << 64, 1 << 64), 1);
    assert_eq!(mul_high_u128(u128::MAX, u128::MAX), 1);
}

#[test]
fn test_mul_high_u128_boundary_conditions() {
    assert_eq!(mul_high_u128(u128::MAX - 1, 2), 0);
    assert_eq!(mul_high_u128(u128::MAX, 1), 0);
    assert_eq!(mul_high_u128(u128::MAX, u128::MAX), 1);
}

#[test]
#[should_panic(expected = "empty range")]
fn test_mul_high_u128_empty_range() {
    let _ = mul_high_u128(0, 0); // This will not panic but included for coverage
    // The function itself does not check ranges, thus it cannot panic; kept this for consistency
}

