// Answer 0

#[test]
fn test_new_invalid_length_too_short() {
    let result = base64::new("ABCD");
    assert_eq!(result, Err(base64::ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_invalid_length_too_long() {
    let result = base64::new("ABCDEFGHIJKLMNOPQRSTUVWXYZA");
    assert_eq!(result, Err(base64::ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_invalid_length_empty() {
    let result = base64::new("");
    assert_eq!(result, Err(base64::ParseAlphabetError::InvalidLength));
}

