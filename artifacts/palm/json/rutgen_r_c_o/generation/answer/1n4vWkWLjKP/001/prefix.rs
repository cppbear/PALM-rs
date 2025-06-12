// Answer 0

#[test]
fn test_float_key_must_be_finite() {
    float_key_must_be_finite();
}

#[test]
fn test_float_key_must_be_finite_with_zero_parameters() {
    Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0);
}

