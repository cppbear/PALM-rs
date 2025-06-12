// Answer 0

#[test]
fn test_d2d_exponent_zero_mantissa_not_zero() {
    let ieee_mantissa: u64 = 1; // Non-zero mantissa
    let ieee_exponent: u32 = 0; // Exponent is zero

    let result = d2d(ieee_mantissa, ieee_exponent);
    
    assert_eq!(result.mantissa, 4); // Expected mantissa
    assert_eq!(result.exponent, -1071); // Expected exponent based on the logic
}

#[test]
#[should_panic]
fn test_d2d_exponent_zero_mantissa_zero() {
    let ieee_mantissa: u64 = 0; // This should trigger panic due to mantissa being zero
    let ieee_exponent: u32 = 0;

    let _ = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_edge_case() {
    let ieee_mantissa: u64 = 0xFFFFFFFFFFFFFFFF; // Max non-zero mantissa
    let ieee_exponent: u32 = 0; // Exponent is still zero

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert!(result.mantissa > 0); // Checking that the mantissa is a valid positive output
    assert_eq!(result.exponent, -1071); // Revised value according to the assertions made
}

#[test]
#[should_panic]
fn test_d2d_invalid_q_condition() {
    let ieee_mantissa: u64 = 1; // Non-zero mantissa
    let ieee_exponent: u32 = 2048; // Chosen to exceed the length of DOUBLE_POW5_INV_SPLIT

    let _ = d2d(ieee_mantissa, ieee_exponent);
}

