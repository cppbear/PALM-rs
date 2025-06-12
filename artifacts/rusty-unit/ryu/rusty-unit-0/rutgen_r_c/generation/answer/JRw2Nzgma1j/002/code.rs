// Answer 0

#[test]
#[should_panic]
fn test_pow5_factor_zero() {
    let value: u64 = 0; // This will make the debug_assert!(value != 0) fail.
    let _ = pow5_factor(value);
}

#[test]
fn test_pow5_factor_small_non_zero() {
    let value: u64 = 1; // A small positive value that should not panic.
    let result = pow5_factor(value);
    assert_eq!(result, 0); // Expecting 0 since 1 is not divisible by 5.
}

#[test]
fn test_pow5_factor_small_power_of_five() {
    let value: u64 = 5; // A small value that is a power of 5.
    let result = pow5_factor(value);
    assert_eq!(result, 1); // 5 can be divided by 5 once.
}

#[test]
fn test_pow5_factor_large_non_power_of_five() {
    let value: u64 = 20; // A value that can be divided multiple times.
    let result = pow5_factor(value);
    assert_eq!(result, 1); // 20 can be divided by 5 once (20 -> 4).
}

#[test]
fn test_pow5_factor_large_power_of_five() {
    let value: u64 = 125; // 5^3
    let result = pow5_factor(value);
    assert_eq!(result, 3); // 125 can be divided by 5 three times.
}

