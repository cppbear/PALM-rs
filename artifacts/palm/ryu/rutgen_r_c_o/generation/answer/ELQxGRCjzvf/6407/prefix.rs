// Answer 0

#[test]
fn test_d2d_case_1() {
    let ieee_mantissa: u64 = 1; // ieee_mantissa != 0
    let ieee_exponent: u32 = 0; // ieee_exponent == 0
    let _result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_2() {
    let ieee_mantissa: u64 = 1; // ieee_mantissa != 0
    let ieee_exponent: u32 = 16; // e2 >= 0 is false
    let _result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_3() {
    let ieee_mantissa: u64 = 1; // ieee_mantissa != 0
    let ieee_exponent: u32 = 1024; // e2 >= 0 is false
    let _result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_4() {
    let ieee_mantissa: u64 = 1; // ieee_mantissa != 0
    let ieee_exponent: u32 = 2048; // e2 >= 0 is false
    let _result = d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_5() {
    let ieee_mantissa: u64 = 1; // ieee_mantissa != 0
    let ieee_exponent: u32 = 2024; // i < DOUBLE_POW5_SPLIT.len() as i32 is true
    let _result = d2d(ieee_mantissa, ieee_exponent);
}

