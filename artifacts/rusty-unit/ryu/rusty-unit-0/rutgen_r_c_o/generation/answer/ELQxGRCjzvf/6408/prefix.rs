// Answer 0

#[test]
fn test_d2d_ieee_exponent_0_ieee_mantissa_0() {
    let ieee_mantissa: u64 = 0;
    let ieee_exponent: u32 = 0;
    d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_ieee_exponent_0_ieee_mantissa_1() {
    let ieee_mantissa: u64 = 1;
    let ieee_exponent: u32 = 0;
    d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_ieee_exponent_2047_ieee_mantissa_0() {
    let ieee_mantissa: u64 = 0;
    let ieee_exponent: u32 = 2047;
    d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_ieee_exponent_2048_ieee_mantissa_0() {
    let ieee_mantissa: u64 = 0;
    let ieee_exponent: u32 = 2048;
    d2d(ieee_mantissa, ieee_exponent);
}

#[test]
fn test_d2d_ieee_exponent_2048_ieee_mantissa_large() {
    let ieee_mantissa: u64 = 4503599627370496;
    let ieee_exponent: u32 = 2048;
    d2d(ieee_mantissa, ieee_exponent);
}

