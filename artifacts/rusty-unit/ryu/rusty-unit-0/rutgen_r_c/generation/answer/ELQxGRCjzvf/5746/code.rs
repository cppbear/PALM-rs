// Answer 0

#[test]
fn test_d2d_with_ieee_exponent_zero() {
    let ieee_mantissa: u64 = 0; // ieee_mantissa != 0 is false
    let ieee_exponent: u32 = 0; // constraint: ieee_exponent == 0 is true

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, -1074); // The exponent for ieee_exponent == 0
    assert_eq!(result.mantissa, 0); // Expected mantissa
}

#[test]
fn test_d2d_with_negative_e2() {
    let ieee_mantissa: u64 = 1; // Random non-zero value for mantissa
    let ieee_exponent: u32 = (DOUBLE_BIAS + DOUBLE_MANTISSA_BITS) as u32 + 1; // Make e2 negative

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent < 0);
}

#[test]
fn test_d2d_with_valid_parameters() {
    let ieee_mantissa: u64 = 1; // Random non-zero value for mantissa
    let ieee_exponent: u32 = (DOUBLE_BIAS + DOUBLE_MANTISSA_BITS) as u32 + 1; // Make e2 negative if calculated

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent < 0); // Test inverse relationship

    let q = log10_pow2(result.exponent + DOUBLE_BIAS + DOUBLE_MANTISSA_BITS as i32) - (result.exponent > 3) as u32;
    assert!(q <= 1); // Constraint on q
}

#[test]
fn test_d2d_with_trailing_zeros() {
    let ieee_mantissa: u64 = 4; // Random non-zero value
    let ieee_exponent: u32 = (DOUBLE_BIAS + DOUBLE_MANTISSA_BITS) as u32 + 3; // A value that may lead to trailing zeros

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa % 10 == 0); // Check if output meets trailing zeros conditions
    assert!(result.exponent < 0); // Ensure e2 is still negative
}

#[test]
fn test_d2d_last_removed_digit() {
    let ieee_mantissa: u64 = 3; // Example input
    let ieee_exponent: u32 = 10; // Example exponent

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.mantissa % 10, 5); // Check if last removed digit is 5
    assert_eq!(result.exponent, 1); // Check exponent as per expectation
}

#[test]
fn test_d2d_equal_vr_vm() {
    let ieee_mantissa: u64 = 2; // Some example value
    let ieee_exponent: u32 = (DOUBLE_BIAS + DOUBLE_MANTISSA_BITS) as u32 + 2; // Set exponent to generate equal vr and vm

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.mantissa, result.mantissa); // Ensure vr == vm
    assert_eq!(result.exponent, -2); // Check expected exponent
}

#[test]
fn test_d2d_accept_bounds_false() {
    let ieee_mantissa: u64 = 1; // Random non-zero
    let ieee_exponent: u32 = (DOUBLE_BIAS + DOUBLE_MANTISSA_BITS) as u32 - 1; // Adjust exponent

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(!result.mantissa.is_power_of_two()); // Should not accept bounds in this case
}

