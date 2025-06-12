// Answer 0

#[test]
fn test_key_must_be_a_string() {
    use serde_json::Error;
    use serde_json::ErrorCode;

    let error = key_must_be_a_string();
    assert_eq!(error.code(), ErrorCode::KeyMustBeAString);
    assert_eq!(error.line(), 0);
    assert_eq!(error.column(), 0);
}

