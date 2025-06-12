// Answer 0

#[test]
fn test_d2d_case_1() {
    let ieee_mantissa: u64 = 0b1111111111111111111111111111111111111111111111111111; // Example non-zero mantissa
    let ieee_exponent: u32 = 2047; // Maximum exponent for a normalized double
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, 8); // Expected exponent based on the computation
    assert_eq!(result.mantissa, 18446744073709551615); // Expected mantissa based on the computation
}

#[test]
fn test_d2d_case_2() {
    let ieee_mantissa: u64 = 0b1010101010101010101010101010101010101010101010101010; // Another example non-zero mantissa
    let ieee_exponent: u32 = 2046; // Another exponent just below the maximum
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, 7); // Example expected exponent
    assert_eq!(result.mantissa, 9844232158178587162); // Example expected mantissa
}

#[test]
fn test_d2d_case_3() {
    let ieee_mantissa: u64 = 0b1111111111111111111111111111111111111111111111111111; // Non-zero mantissa
    let ieee_exponent: u32 = 2045; // Exponent below the maximum
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.exponent > 0); // Ensuring exponent is positive based on computation
    assert!(result.mantissa > 1); // Ensuring mantissa is more than 1
}

#[test]
fn test_d2d_case_4() {
    let ieee_mantissa: u64 = 0b1101101101101101101101101101101101101101101101101101; // A complex non-zero mantissa
    let ieee_exponent: u32 = 2044; // Another maximum exponent check
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_ne!(result.exponent, 1); // Ensuring exponent isn't equal to 1
    assert_ne!(result.mantissa, 0); // Resulting mantissa should not be zero
}

#[test]
fn test_d2d_case_5() {
    let ieee_mantissa: u64 = 0b1010111101110011110011110011110011110011110011110011; // Yet another non-zero mantissa
    let ieee_exponent: u32 = 2043; // Exponent just above the mid-point
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert!(result.mantissa < u64::MAX); // Ensuring mantissa is within valid range
}

