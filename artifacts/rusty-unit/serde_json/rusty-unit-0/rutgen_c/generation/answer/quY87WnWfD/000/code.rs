// Answer 0

#[test]
fn test_key_must_be_a_string() {
    let error = key_must_be_a_string();
    assert_eq!(matches!(error.err.code, ErrorCode::KeyMustBeAString), true);
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
}

