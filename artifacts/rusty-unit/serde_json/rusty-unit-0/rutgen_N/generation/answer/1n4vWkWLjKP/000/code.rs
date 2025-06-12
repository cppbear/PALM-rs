// Answer 0

#[cfg(test)]
fn float_key_must_be_finite() -> Error {
    Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0)
}

#[test]
fn test_float_key_must_be_finite() {
    let error = float_key_must_be_finite();
    assert_eq!(error.code, ErrorCode::FloatKeyMustBeFinite);
    assert_eq!(error.line, 0);
    assert_eq!(error.column, 0);
}

