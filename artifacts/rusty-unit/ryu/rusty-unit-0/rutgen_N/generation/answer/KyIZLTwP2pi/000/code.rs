// Answer 0

#[test]
fn test_is_nonfinite() {
    struct FloatWrapper(f32);
    
    impl FloatWrapper {
        fn to_bits(self) -> u32 {
            self.0.to_bits()
        }

        fn is_nonfinite(self) -> bool {
            const EXP_MASK: u32 = 0x7f800000;
            let bits = self.to_bits();
            bits & EXP_MASK == EXP_MASK
        }
    }

    // Test for NaN
    let nan_float = FloatWrapper(f32::NAN);
    assert!(!nan_float.is_nonfinite());

    // Test for positive infinity
    let pos_infinity_float = FloatWrapper(f32::INFINITY);
    assert!(pos_infinity_float.is_nonfinite());

    // Test for negative infinity
    let neg_infinity_float = FloatWrapper(f32::NEG_INFINITY);
    assert!(neg_infinity_float.is_nonfinite());

    // Test for a finite value
    let finite_float = FloatWrapper(1.0);
    assert!(!finite_float.is_nonfinite());

    // Test for another finite value
    let another_finite_float = FloatWrapper(-5.5);
    assert!(!another_finite_float.is_nonfinite());
}

