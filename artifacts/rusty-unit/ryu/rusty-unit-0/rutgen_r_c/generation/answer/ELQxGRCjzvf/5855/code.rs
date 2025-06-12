// Answer 0

#[test]
fn test_d2d_zero_exponent() {
    let ieee_mantissa: u64 = 1; // non-zero mantissa
    let ieee_exponent: u32 = 0; // zero exponent

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent < 0); // since e2 will be negative
    assert_eq!(result.mantissa, 1); // mantissa should remain the same
}

#[test]
fn test_d2d_negative_exponent() {
    let ieee_mantissa: u64 = 0b1; // non-zero mantissa
    let ieee_exponent: u32 = 1; // will yield a negative exponent after adjustment

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent < 0); // e2 < 0 due to adjustment
    assert_eq!(result.mantissa, 2); // should calculate the corresponding mantissa correctly
}

#[test]
fn test_d2d_invalid_panic_double_pow5_split() {
    let ieee_mantissa: u64 = 2; // valid mantissa
    let ieee_exponent: u32 = 2047; // max exponent to avoid panic

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent >= 0); // should be handled safely
}

#[test]
fn test_d2d_q_leq_one() {
    let ieee_mantissa: u64 = 4; // a valid mantissa
    let ieee_exponent: u32 = 3; // adjust for testing q <= 1 constraint

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, 0); // check for expected exponent after transformation
    assert_eq!(result.mantissa, 1); // Check if it returns correct mantissa
}

#[test]
fn test_d2d_vp_vm_division() {
    let ieee_mantissa: u64 = 12; // mantissa affects the division
    let ieee_exponent: u32 = 4; // exponent that ensures the condition holds true

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa > 0);
    assert_eq!(result.mantissa % 10, 0); // should return a rounded mantissa
}

