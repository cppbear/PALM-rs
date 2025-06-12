// Answer 0

#[test]
fn test_d2d_case_1() {
    let ieee_mantissa: u64 = 1;
    let ieee_exponent: u32 = 1023;
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_2() {
    let ieee_mantissa: u64 = (1u64 << DOUBLE_MANTISSA_BITS) - 1; // max value for mantissa
    let ieee_exponent: u32 = 1024; // just above bias
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_3() {
    let ieee_mantissa: u64 = 2; // a small non-zero value
    let ieee_exponent: u32 = 2047; // max exponent
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_4() {
    let ieee_mantissa: u64 = 1_000_000_000; // a larger mantissa
    let ieee_exponent: u32 = 2047; // max exponent
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_5() {
    let ieee_mantissa: u64 = 10; // simple small mantissa
    let ieee_exponent: u32 = 1084; // just below bias
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_6() {
    let ieee_mantissa: u64 = 999_999_999; // a large mantissa
    let ieee_exponent: u32 = 1024; // slightly above bias
    let result = d2d(ieee_mantissa, ieee_exponent);
}

