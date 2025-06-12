// Answer 0

#[test]
fn test_new_invalid_length_too_short() {
    let alphabet = "ABCDE"; // Length 5, should trigger InvalidLength
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_invalid_length_too_long() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="; // Length 66, should trigger InvalidLength
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_invalid_length_exact() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // Length 64, valid case
    let result = Alphabet::new(alphabet);
    assert!(result.is_ok());
}

