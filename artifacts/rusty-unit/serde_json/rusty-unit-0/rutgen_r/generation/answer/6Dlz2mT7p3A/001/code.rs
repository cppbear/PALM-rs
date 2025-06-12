// Answer 0

fn float_key_must_be_finite_test() {
    let error = float_key_must_be_finite();
    assert_eq!(error, Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0));
}

#[test]
fn test_float_key_must_be_finite() {
    float_key_must_be_finite_test();
}

