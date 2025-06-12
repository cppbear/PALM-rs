// Answer 0

#[test]
fn test_d2d_exponent_zero_mantissa_non_zero() {
    let ieee_mantissa = 1; // Non-zero mantissa
    let ieee_exponent = 0; // Exponent is zero
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.mantissa, 1); // Expected mantissa
    assert_eq!(result.exponent, 1020); // Evaluated from the logic in the function
}

#[test]
#[should_panic] // Expected to panic because ieee_exponent is zero and should not be.
fn test_d2d_invalid_mantissa_zero() {
    let ieee_mantissa = 0; // Zero mantissa, invalid value
    let ieee_exponent = 0; // Exponent is zero
    let _result = d2d(ieee_mantissa, ieee_exponent); // Should panic
}

#[test]
#[should_panic] // Expected to panic because e2 >= 0 is false (negative exponent).
fn test_d2d_negative_exponent() {
    let ieee_mantissa = 1; // Non-zero mantissa
    let ieee_exponent = 1022; // This will cause e2 to be negative
    let _result = d2d(ieee_mantissa, ieee_exponent); // Should panic
}

#[test]
#[should_panic] // Expected to panic because i < DOUBLE_POW5_SPLIT.len() as i32 is false.
fn test_d2d_invalid_double_pow5_split_index() {
    let ieee_mantissa = 1; // Non-zero mantissa
    let ieee_exponent = 2047; // This should cause i to equal DOUBLE_POW5_SPLIT.len() as i32
    let _result = d2d(ieee_mantissa, ieee_exponent); // Should panic
}

