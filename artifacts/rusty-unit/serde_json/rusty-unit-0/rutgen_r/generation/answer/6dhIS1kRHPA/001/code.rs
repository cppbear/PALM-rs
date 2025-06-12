// Answer 0

#[test]
fn test_key_must_be_a_string() {
    let error = key_must_be_a_string();
    match error {
        Error::Syntax(code, line, col) => {
            assert_eq!(code, ErrorCode::KeyMustBeAString);
            assert_eq!(line, 0);
            assert_eq!(col, 0);
        },
        _ => panic!("Expected syntax error with KeyMustBeAString"),
    }
}

