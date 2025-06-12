// Answer 0

#[test]
fn test_f64_from_parts_zero_significand_negative_exponent() {
    let mut deserializer = Deserializer {
        read: /* appropriate Read implementation */,
        scratch: Vec::new(),
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let positive = false;
    let significand = 0;
    let exponent = -309;

    let _ = deserializer.f64_from_parts(positive, significand, exponent);
}

#[test]
fn test_f64_from_parts_zero_significand_negative_exponent_edge() {
    let mut deserializer = Deserializer {
        read: /* appropriate Read implementation */,
        scratch: Vec::new(),
        remaining_depth: 1,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let positive = false;
    let significand = 0;
    let exponent = -308;

    let _ = deserializer.f64_from_parts(positive, significand, exponent);
}

