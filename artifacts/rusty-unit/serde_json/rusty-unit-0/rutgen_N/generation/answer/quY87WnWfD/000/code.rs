// Answer 0

#[test]
fn test_key_must_be_a_string() {
    use serde_json::error::{Error, ErrorCode};

    let error = key_must_be_a_string();
    match error {
        Error::Syntax(err_code, _, _) => {
            assert_eq!(err_code, ErrorCode::KeyMustBeAString);
        },
        _ => panic!("Expected Syntax error"),
    }
}

