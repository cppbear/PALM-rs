// Answer 0

#[test]
fn test_d2d_zero_exponent() {
    let ieee_mantissa: u64 = 0; // Should be non-zero to avoid putting ieee_exponent == 0 as true
    let ieee_exponent: u32 = 0; 

    let result = d2d(ieee_mantissa, ieee_exponent);
    
    assert_eq!(result.mantissa, 0); // Expected mantissa should be 0
    assert_eq!(result.exponent, 0); // Expected exponent should be 0
}

#[test]
fn test_d2d_non_zero_mantissa() {
    let ieee_mantissa: u64 = 1; // Non-zero mantissa
    let ieee_exponent: u32 = 0; 

    let result = d2d(ieee_mantissa, ieee_exponent);
    
    assert!(result.mantissa > 0); // Verify that mantissa is greater than 0
    assert_eq!(result.exponent, 0); // Expected exponent should be 0
}

#[test]
fn test_d2d_exponent_positive() {
    let ieee_mantissa: u64 = 1; 
    let ieee_exponent: u32 = 1023; // A positive exponent for a normal float

    let result = d2d(ieee_mantissa, ieee_exponent);
    
    assert!(result.mantissa > 0); // Verify that mantissa is greater than 0
    assert!(result.exponent >= 0); // Expected exponent should be non-negative
}

#[test]
fn test_d2d_exponent_high() {
    let ieee_mantissa: u64 = 1; 
    let ieee_exponent: u32 = 2047; // High exponent close to overflow

    let result = d2d(ieee_mantissa, ieee_exponent);
    
    assert!(result.mantissa > 0); // Ensure mantissa is valid
    assert!(result.exponent >= 0); // Ensure exponent is valid
}

#[test]
fn test_d2d_check_conditions() {
    let ieee_mantissa: u64 = 999999999999; // A large non-zero mantissa
    let ieee_exponent: u32 = 1024; // A valid exponent

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert!(result.mantissa > 0); // Non-zero mantissa expected
    assert!(result.exponent <= 100); // Check if exponent is within expected range
}

#[test]
fn test_d2d_rounding_behavior() {
    let ieee_mantissa: u64 = 10; // Non-zero mantissa
    let ieee_exponent: u32 = 1024; // Valid positive exponent

    let result = d2d(ieee_mantissa, ieee_exponent);
    
    assert!(result.mantissa > 10); // Ensuring that rounding occurs
    assert!(result.exponent > 0); // Check for change in exponent
}

