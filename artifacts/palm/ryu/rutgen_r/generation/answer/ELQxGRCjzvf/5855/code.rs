// Answer 0

#[test]
fn test_d2d_case1() {
    struct FloatingDecimal64 {
        exponent: i32,
        mantissa: u64,
    }

    const DOUBLE_BIAS: i32 = 1023;
    const DOUBLE_MANTISSA_BITS: i32 = 52;
    const DOUBLE_POW5_INV_SPLIT: [u64; 22] = [1; 22];
    
    let ieee_mantissa: u64 = 0;
    let ieee_exponent: u32 = 0; // constraint: ieee_exponent == 0 is true

    let result = d2d(ieee_mantissa, ieee_exponent);
    
    assert_eq!(result.exponent, 1 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS - 2);
    assert_eq!(result.mantissa, ieee_mantissa); // Using ieee_mantissa which is 0
}

#[test]
fn test_d2d_case2() {
    struct FloatingDecimal64 {
        exponent: i32,
        mantissa: u64,
    }

    const DOUBLE_BIAS: i32 = 1023;
    const DOUBLE_MANTISSA_BITS: i32 = 52;
    const DOUBLE_POW5_SPLIT: [u64; 22] = [5; 22]; // ensuring we access valid data 

    let ieee_mantissa: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111; // Simulate a non-zero mantissa.
    let ieee_exponent: u32 = 1; // Non-zero exponent but may affect q's computation.

    let result = d2d(ieee_mantissa, ieee_exponent);
    
    assert!(result.mantissa != ieee_mantissa); // Expecting different output on valid inputs.
}

#[test]
fn test_d2d_case3() {
    struct FloatingDecimal64 {
        exponent: i32,
        mantissa: u64,
    }

    const DOUBLE_BIAS: i32 = 1023;
    const DOUBLE_MANTISSA_BITS: i32 = 52;
    const DOUBLE_POW5_SPLIT: [u64; 22] = [10; 22]; // ensures valid access manipulations

    let ieee_mantissa: u64 = 0b11000011011; // A representative non-zero mantissa.
    let ieee_exponent: u32 = 8; // Just a sample exponent

    let result = d2d(ieee_mantissa, ieee_exponent);

    assert!(result.exponent >= 0); // Ensuring we cover the e2 >= 0 condition.
    assert!(result.mantissa > 0); // Output mantissa should be valid and possibly different as well.
}

