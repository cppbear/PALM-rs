// Answer 0

#[test]
fn test_float_key_must_be_finite() {
    use serde_json::Error;
    use serde_json::ErrorCode;

    let error = float_key_must_be_finite();
    assert_eq!(error, Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0));
}

