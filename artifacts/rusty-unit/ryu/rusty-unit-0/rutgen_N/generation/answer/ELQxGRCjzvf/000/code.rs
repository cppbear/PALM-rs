// Answer 0

#[test]
fn test_d2d_zero_mantissa_zero_exponent() {
    let ieee_mantissa = 0;
    let ieee_exponent = 0;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.mantissa, 0);
    assert_eq!(result.exponent, 0);
}

#[test]
fn test_d2d_non_zero_mantissa_zero_exponent() {
    let ieee_mantissa = 1;
    let ieee_exponent = 0;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa > 0);
    assert_eq!(result.exponent, -1074); // Using DOUBLE_BIAS - 1023 - 2
}

#[test]
fn test_d2d_large_mantissa() {
    let ieee_mantissa = 0xFFFFFFFFFFFFFFFF;
    let ieee_exponent = 2047; // Max exponent
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa > 0);
}

#[test]
fn test_d2d_small_positive_exponent() {
    let ieee_mantissa = 1;
    let ieee_exponent = 1; // Minimum positive exponent
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa > 0);
}

#[test]
fn test_d2d_small_negative_exponent() {
    let ieee_mantissa = 1;
    let ieee_exponent = 0xFFFFFFFF; // Represents a large negative exponent
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa > 0);
}

