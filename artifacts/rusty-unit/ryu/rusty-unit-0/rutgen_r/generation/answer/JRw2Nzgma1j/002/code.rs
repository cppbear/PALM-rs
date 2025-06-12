// Answer 0

#[test]
#[should_panic]
fn test_pow5_factor_zero() {
    let value: u64 = 0; // This input should trigger a panic due to the debug_assert!(value != 0);
    let _ = pow5_factor(value);
}

#[test]
fn test_pow5_factor_small_value() {
    let value: u64 = 5; // Smallest value that is a multiple of 5, expected output is 1
    let result = pow5_factor(value);
    assert_eq!(result, 1);
}

#[test]
fn test_pow5_factor_large_value() {
    let value: u64 = 3125; // 5^5, expected output is 5
    let result = pow5_factor(value);
    assert_eq!(result, 5);
}

#[test]
fn test_pow5_factor_boundary() {
    let value: u64 = 1 << 63; // A large number, should not exceed iterations allowed, expected output is 0
    let result = pow5_factor(value);
    assert_eq!(result, 0);
}

#[test]
fn test_pow5_factor_max_u64() {
    let value: u64 = u64::MAX; // 18446744073709551615, expected output should be checked based on iterations
    let result = pow5_factor(value);
    assert!(result > 0);
}

