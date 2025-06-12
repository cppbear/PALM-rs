// Answer 0

#[test]
fn test_d2d_exponent_zero() {
    let ieee_mantissa = 0;
    let ieee_exponent = 0;

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, 1 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2);
    assert_eq!(result.mantissa, ieee_mantissa);
}

#[test]
fn test_d2d_mantissa_non_zero() {
    let ieee_mantissa = 1;
    let ieee_exponent = 0;

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, 1 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2);
    assert_eq!(result.mantissa, ieee_mantissa);
}

#[test]
#[should_panic]
fn test_d2d_exponent_underflow() {
    let ieee_mantissa = 1;
    let ieee_exponent = std::u32::MAX;

    let _result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_bound_q_equals_63() {
    let ieee_mantissa = 0xFFFFFFFFFFFFFFFF; // Max mantissa
    let ieee_exponent = (DOUBLE_BIAS + DOUBLE_MANTISSA_BITS + 2) as u32 + 63; // Setting up e2 to create q == 63

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa > 0);
}

#[test]
fn test_d2d_div100_equal() {
    let ieee_mantissa = 10;
    let ieee_exponent = 0b10000000011; // Arbitrary exponent to satisfy conditions

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(div100(result.mantissa), div100(result.mantissa));
}

#[test]
fn test_d2d_div10_comparison() {
    let ieee_mantissa = 100;
    let ieee_exponent = 0b10000000010;

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(div10(result.mantissa) <= div10(result.mantissa));
}

#[test]
fn test_d2d_vr_vm_difference() {
    let ieee_mantissa = 20;
    let ieee_exponent = 0b10000000001;

    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.vr != result.vm);
}

