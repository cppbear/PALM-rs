// Answer 0

#[test]
fn test_d2d_case1() {
    struct FloatingDecimal64 {
        exponent: i32,
        mantissa: u64,
    }

    const DOUBLE_BIAS: i32 = 1023;
    const DOUBLE_MANTISSA_BITS: u32 = 52;
    const DOUBLE_POW5_INV_SPLIT: [u64; 100] = [1; 100]; // Simplified for testing purposes
    const DOUBLE_POW5_BITCOUNT: i32 = 24; // Placeholder value

    fn d2d(ieee_mantissa: u64, ieee_exponent: u32) -> FloatingDecimal64 {
        // Implementation from your provided function
        // ...
        // (Omitted for brevity, paste the entire `d2d` function here)
        todo!()
    }

    // Test inputs based on provided constraints
    let ieee_mantissa: u64 = 0; // Should be false for the provided contraint
    let ieee_exponent: u32 = 0;  // Should match the case where ieee_exponent == 0

    let result = d2d(ieee_mantissa, ieee_exponent);
    
    // Expected output checking
    assert_eq!(result.exponent, /* expected exponent value here based on constraints */);
    assert_eq!(result.mantissa, /* expected mantissa value here based on constraints */);
}

#[test]
#[should_panic]
fn test_d2d_case2() {
    struct FloatingDecimal64 {
        exponent: i32,
        mantissa: u64,
    }

    fn d2d(ieee_mantissa: u64, ieee_exponent: u32) -> FloatingDecimal64 {
        // Implementation from your provided function
        // ...
        // (Omitted for brevity, paste the entire `d2d` function here)
        todo!()
    }

    let ieee_mantissa: u64 = 1; // Triggers panic due to ieee_mantissa expected to be zero
    let ieee_exponent: u32 = 0;

    d2d(ieee_mantissa, ieee_exponent);
}

