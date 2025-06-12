// Answer 0

#[test]
fn test_float_key_must_be_finite() {
    let error = float_key_must_be_finite();
    assert_eq!(if let ErrorCode::FloatKeyMustBeFinite = error.err.code {
        true
    } else {
        false
    }, true);
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
}

#[test]
fn test_float_key_must_be_finite_properties() {
    let error = float_key_must_be_finite();
    assert!(matches!(error.err.code, ErrorCode::FloatKeyMustBeFinite));
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
}

