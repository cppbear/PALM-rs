// Answer 0

#[test]
fn test_format_nonfinite_nan() {
    struct FloatWrapper {
        value: f32,
    }

    impl FloatWrapper {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }
    }
    
    let nan_value = FloatWrapper { value: f32::NAN };
    assert_eq!(nan_value.format_nonfinite(), "NaN");
}

#[test]
fn test_format_nonfinite_negative_infinity() {
    struct FloatWrapper {
        value: f32,
    }

    impl FloatWrapper {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }
    }
    
    let neg_infinity_value = FloatWrapper { value: f32::NEG_INFINITY };
    assert_eq!(neg_infinity_value.format_nonfinite(), "-Infinity");
}

#[test]
fn test_format_nonfinite_infinity() {
    struct FloatWrapper {
        value: f32,
    }

    impl FloatWrapper {
        fn to_bits(self) -> u32 {
            self.value.to_bits()
        }
    }
    
    let infinity_value = FloatWrapper { value: f32::INFINITY };
    assert_eq!(infinity_value.format_nonfinite(), "Infinity");
}

