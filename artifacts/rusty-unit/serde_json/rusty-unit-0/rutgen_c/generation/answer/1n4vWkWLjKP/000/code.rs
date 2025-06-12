// Answer 0

#[test]
fn test_float_key_must_be_finite() {
    let error = float_key_must_be_finite();
    match error.err.code {
        ErrorCode::FloatKeyMustBeFinite => {},
        _ => panic!("Expected FloatKeyMustBeFinite error code"),
    }
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
}

