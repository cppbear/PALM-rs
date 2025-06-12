// Answer 0

#[test]
fn test_d2d_case_with_constraints() {
    struct TestInputs {
        ieee_mantissa: u64,
        ieee_exponent: u32,
    }

    let inputs = TestInputs {
        ieee_mantissa: 0, // ieee_mantissa must not be 0
        ieee_exponent: 0, // ieee_exponent must be 0
    };

    let result = d2d(inputs.ieee_mantissa, inputs.ieee_exponent);
    
    // Check the values in the result for the expected properties
    assert_eq!(result.exponent, -1022); // e2 == 0 implies exponent is -1022
    assert!(result.mantissa != 0); // Check that mantissa is not zero
}

#[test]
#[should_panic]
fn test_d2d_invalid_case() {
    struct TestInputs {
        ieee_mantissa: u64,
        ieee_exponent: u32,
    }

    // An invalid test case should cause a panic
    let inputs = TestInputs {
        ieee_mantissa: 1,  // Use a safe value
        ieee_exponent: 2048, // This should panic since 2048 is outside valid boundary
    };

    let _ = d2d(inputs.ieee_mantissa, inputs.ieee_exponent);
}

#[test]
fn test_d2d_small_increment_mantissa() {
    struct TestInputs {
        ieee_mantissa: u64,
        ieee_exponent: u32,
    }

    let inputs = TestInputs {
        ieee_mantissa: 2, // A small non-zero mantissa
        ieee_exponent: 1, // Small exponent
    };

    let result = d2d(inputs.ieee_mantissa, inputs.ieee_exponent);
    
    assert_eq!(result.exponent, -1021); // Expecting modified exponent
    assert!(result.mantissa != 0); // Ensure that we have a non-zero mantissa
}

#[test]
fn test_d2d_large_values_case() {
    struct TestInputs {
        ieee_mantissa: u64,
        ieee_exponent: u32,
    }

    let inputs = TestInputs {
        ieee_mantissa: 0b1111111111111111111111111111111111111111111111111111111111111111, // Max mantissa
        ieee_exponent: 2047, // Max valid exponent
    };

    let result = d2d(inputs.ieee_mantissa, inputs.ieee_exponent);
    
    assert!(result.mantissa > 0); // Ensure that the mantissa is positive
    assert!(result.exponent != 0); // Exponent has proper adjustments
}

#[test]
fn test_d2d_zero_exponent_invalid() {
    struct TestInputs {
        ieee_mantissa: u64,
        ieee_exponent: u32,
    }

    // This should hit the boundary condition of exponent being zero
    let inputs = TestInputs {
        ieee_mantissa: 0, // Should not be zero
        ieee_exponent: 0, 
    };

    let _ = d2d(inputs.ieee_mantissa, inputs.ieee_exponent);
}

#[test]
fn test_d2d_case_with_even_mantissa() {
    struct TestInputs {
        ieee_mantissa: u64,
        ieee_exponent: u32,
    }

    // A case with an even mantissa but still valid
    let inputs = TestInputs {
        ieee_mantissa: 8, // Non-zero, even mantissa
        ieee_exponent: 3, // Normal range value
    };

    let result = d2d(inputs.ieee_mantissa, inputs.ieee_exponent);
    
    assert_eq!(result.mantissa % 2, 0); // Should be even
    assert!(result.mantissa != 0); // Non-zero mantissa is required
}

