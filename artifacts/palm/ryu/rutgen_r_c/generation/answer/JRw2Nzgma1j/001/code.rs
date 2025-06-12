// Answer 0

#[test]
fn test_pow5_factor_non_zero() {
    let value = 1; // Smallest non-zero value
    let result = pow5_factor(value);
    assert_eq!(result, 0); // Expect count to be 0 since value is not multiplied
}

#[test]
fn test_pow5_factor_slightly_above_n_div_5() {
    let value = 3689348814741910324; // Just above N_DIV_5
    let result = pow5_factor(value);
    assert_eq!(result, 0); // Should immediately break out of the loop
}

#[test]
fn test_pow5_factor_boundary_case() {
    let value = 3689348814741910323; // Equal to N_DIV_5
    let result = pow5_factor(value);
    assert_eq!(result, 1); // Should increment count once before breaking the loop
}

#[test]
fn test_pow5_factor_large_value() {
    let value = 10000000000000000000; // A large non-zero value
    let result = pow5_factor(value);
    assert!(result > 0); // Expect count greater than 0 as it will enter the loop
}

#[test]
#[should_panic] // This test is expected to panic as it should not call with value 0
fn test_pow5_factor_zero() {
    let value = 0; // Panic case since value cannot be zero
    let _ = pow5_factor(value);
}

