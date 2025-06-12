// Answer 0

#[test]
fn test_d2d_zero_exponent() {
    let ieee_mantissa = 0;
    let ieee_exponent = 0;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result, FloatingDecimal64 { mantissa: 0, exponent: 0 });
}

#[test]
fn test_d2d_non_zero_mantissa() {
    let ieee_mantissa = 1;
    let ieee_exponent = 0;
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result, FloatingDecimal64 { mantissa: 1, exponent: 0 });
}

#[test]
fn test_d2d_boundary_conditions() {
    // Setting e2=0 for ieee_mantissa = 0x3FF (i.e., 1.0 in double)
    let ieee_mantissa = 0x3FF;
    let ieee_exponent = 1023; // 1023 is the bias for exponent 0
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result, FloatingDecimal64 { mantissa: 1, exponent: 0 });
}

#[test]
fn test_d2d_large_mantissa_exponent() {
    let ieee_mantissa = 0xFFFFFFFFFFFFF; // Max mantissa for a double
    let ieee_exponent = 2046; // Max exponent before overflow
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa > 0);
}

#[test]
fn test_d2d_even_mantissa() {
    let ieee_mantissa = 2; // Even mantissa
    let ieee_exponent = 1023; // Exponent corresponding to 1.0
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result, FloatingDecimal64 { mantissa: 1, exponent: 1 });
}

