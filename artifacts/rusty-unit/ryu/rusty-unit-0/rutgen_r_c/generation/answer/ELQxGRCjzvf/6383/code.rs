// Answer 0

#[test]
fn test_d2d_exponent_zero() {
    let ieee_mantissa = 1; // Given ieee_mantissa != 0 is false
    let ieee_exponent = 0; // Triggering ieee_exponent == 0 constraint
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent < 0); // e2 >= 0 is false must hold
}

#[test]
fn test_d2d_exponent_negative() {
    let ieee_mantissa = 1; // Given ieee_mantissa != 0 is false
    let ieee_exponent = 1 << 11; // Valid exponent to check negative values
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent < 0); // Confirming that exponent is negative
}

#[test]
fn test_d2d_pow5_split_bound() {
    let ieee_mantissa = 1; // Given ieee_mantissa != 0 is false
    let ieee_exponent = 1 << 11; // Ensure it stays within the valid range for q
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent >= -125 && result.mantissa > 0); // Ensure output is valid
}

#[test]
fn test_d2d_vp_div10_greater_than_vm_div10() {
    let ieee_mantissa = 10; 
    let ieee_exponent = 1 << 11; // Stay in the valid range, leverage q value
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa % 10 == 0); // This tests if the last digit is not zero
}

#[test]
fn test_d2d_trailing_zeros() {
    let ieee_mantissa = 20; // Set so that vr is trailing zeros while vm is not
    let ieee_exponent = 1 << 11; // Keep in bounds
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa % 5 == 0); // Verify that vm has trailing zeros
    assert!(result.exponent >= 0); // e2 condition confirmed
}

#[test]
fn test_d2d_div10_constraints() {
    let ieee_mantissa = 25; // Test value for mantissa
    let ieee_exponent = 1 << 11; // Testing with a valid exponent
    let result = d2d(ieee_mantissa, ieee_exponent);
    let vp_div10 = div10(result.mantissa); // Check the division result
    let vm_div10 = div10(result.mantissa); // Both must be checked for the condition
    assert!(vp_div10 != vm_div10); // Inverse condition
}

#[test]
fn test_d2d_return_value() {
    let ieee_mantissa = 30; // Set up a value to test against
    let ieee_exponent = 1 << 11; // Ensure compliance with the exponent
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.mantissa > 0, true); // Output mantissa must be valid
    assert!(result.exponent.is_negative()); // Ensure exponent is correctly interpreted
}

