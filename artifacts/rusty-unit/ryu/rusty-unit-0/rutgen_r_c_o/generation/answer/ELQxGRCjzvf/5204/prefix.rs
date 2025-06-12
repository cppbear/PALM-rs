// Answer 0

#[test]
fn test_d2d_case_1() {
    let ieee_mantissa: u64 = 0x0;
    let ieee_exponent: u32 = 0;
    d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_2() {
    let ieee_mantissa: u64 = 0x1;
    let ieee_exponent: u32 = 1023;
    d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_3() {
    let ieee_mantissa: u64 = 0x10;
    let ieee_exponent: u32 = 1024;
    d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_4() {
    let ieee_mantissa: u64 = 0xFFFFF;
    let ieee_exponent: u32 = 2047;
    d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_5() {
    let ieee_mantissa: u64 = 0x400;
    let ieee_exponent: u32 = 1535;
    d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_6() {
    let ieee_mantissa: u64 = 0x30;
    let ieee_exponent: u32 = 1026;
    d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_7() {
    let ieee_mantissa: u64 = 0x20;
    let ieee_exponent: u32 = 1025;
    d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_8() {
    let ieee_mantissa: u64 = 0x80000;
    let ieee_exponent: u32 = 2040;
    d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_9() {
    let ieee_mantissa: u64 = 0x0F;
    let ieee_exponent: u32 = 1020;
    d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_case_10() {
    let ieee_mantissa: u64 = 0xFFFFF0;
    let ieee_exponent: u32 = 2044;
    d2d(ieee_mantissa, ieee_exponent);
}

