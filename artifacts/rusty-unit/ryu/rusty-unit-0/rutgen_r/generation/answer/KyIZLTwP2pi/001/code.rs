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

    // Test with positive infinity
    let positive_infinity = FloatWrapper(f32::INFINITY);
    assert!(positive_infinity.is_nonfinite());

    // Test with negative infinity
    let negative_infinity = FloatWrapper(f32::NEG_INFINITY);
    assert!(negative_infinity.is_nonfinite());

    // Test with NaN
    let nan_value = FloatWrapper(f32::NAN);
    assert!(nan_value.is_nonfinite());

    // Test with large finite value
    let large_finite = FloatWrapper(1e10);
    assert!(!large_finite.is_nonfinite());

    // Test with zero
    let zero_value = FloatWrapper(0.0);
    assert!(!zero_value.is_nonfinite());
}

