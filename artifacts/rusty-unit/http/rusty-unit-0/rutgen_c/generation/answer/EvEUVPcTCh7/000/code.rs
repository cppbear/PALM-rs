// Answer 0

#[test]
fn test_to_str_error_display() {
    let error = ToStrError { _priv: () };
    let result = format!("{}", error);
    assert_eq!(result, "failed to convert header to a str");
}

#[test]
fn test_to_str_error_debug() {
    let error = ToStrError { _priv: () };
    let result = format!("{:?}", error);
    assert_eq!(result, "ToStrError {_priv: ()}");
}

