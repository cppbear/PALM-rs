// Answer 0

#[test]
fn test_d2d_case1() {
    let ieee_mantissa: u64 = 0; // ieee_mantissa == 0
    let ieee_exponent: u32 = 0; // ieee_exponent == 0
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case2() {
    let ieee_mantissa: u64 = 1; // ieee_mantissa != 0
    let ieee_exponent: u32 = 0; // ieee_exponent == 0
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case3() {
    let ieee_mantissa: u64 = 0; // ieee_mantissa == 0
    let ieee_exponent: u32 = 1; // ieee_exponent > 0 -> should modify e2
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case4() {
    let ieee_mantissa: u64 = 0; // ieee_mantissa == 0
    let ieee_exponent: u32 = 2; // ieee_exponent > 0
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case5() {
    let ieee_mantissa: u64 = 1; // ieee_mantissa != 0
    let ieee_exponent: u32 = 1023; // valid exponent for testing
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case6() {
    let ieee_mantissa: u64 = 3; // ieee_mantissa != 0
    let ieee_exponent: u32 = 1024; // another valid exponent for testing
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case7() {
    let ieee_mantissa: u64 = 0; // ieee_mantissa == 0
    let ieee_exponent: u32 = 0; // ieee_exponent == 0
    let result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case8() {
    let ieee_mantissa: u64 = 4; // ieee_mantissa != 0
    let ieee_exponent: u32 = 0; // ieee_exponent == 0
    let result = d2d(ieee_mantissa, ieee_exponent);
}

