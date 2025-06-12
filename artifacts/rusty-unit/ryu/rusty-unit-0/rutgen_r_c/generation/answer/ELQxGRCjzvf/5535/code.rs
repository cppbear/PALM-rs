// Answer 0

#[test]
fn test_d2d_case1() {
    let ieee_mantissa: u64 = 0; // ieee_mantissa != 0 is false
    let ieee_exponent: u32 = 0; // ieee_exponent == 0 is true
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result, FloatingDecimal64 { exponent: -1074, mantissa: 0 }); // e2 == 0
}

#[test]
fn test_d2d_case2() {
    let ieee_mantissa: u64 = 0b1; // This case ensures ieee_mantissa != 0
    let ieee_exponent: u32 = 0; // ieee_exponent == 0 is true
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result, FloatingDecimal64 { exponent: -1074, mantissa: 1 }); // e2 == 0
}

#[test]
fn test_d2d_case3() {
    let ieee_mantissa: u64 = 0b1111111111111111111111111111111111111111111111111111; // High value for mantissa
    let ieee_exponent: u32 = 0b00000000000; // Indicates we are testing e2 that should stay >= 0
    let result = d2d(ieee_mantissa, ieee_exponent);
    assert_eq!(result.exponent, -1024);
    assert!(result.mantissa > 0); // Ensure mantissa is valid and not triggering constraints that would cause panic
} 

#[test]
#[should_panic]
fn test_d2d_panic_case() {
    let ieee_mantissa: u64 = 0; // ieee_mantissa != 0 is false - should panic
    let ieee_exponent: u32 = 1;   // ieee_exponent should not trigger panic
    let _result = d2d(ieee_mantissa, ieee_exponent); // This should panic due to the invalid mantissa condition
}

