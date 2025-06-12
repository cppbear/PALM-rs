// Answer 0

#[test]
fn test_new_valid_alphabet() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = base64::new(alphabet);
    assert!(result.is_ok());
}

#[test]
fn test_new_invalid_length() {
    let alphabet = "ABCD"; // Too short
    let result = base64::new(alphabet);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(base64::ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_invalid_character() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/-\x00"; // Contains unprintable byte
    let result = base64::new(alphabet);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(base64::ParseAlphabetError::UnprintableByte(0)));
}

#[test]
fn test_new_reserved_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+="; // Contains reserved byte `=`
    let result = base64::new(alphabet);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(base64::ParseAlphabetError::ReservedByte(b'=')));
}

#[test]
fn test_new_duplicated_byte() {
    let alphabet = "ABCDEFFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // Duplicate 'F'
    let result = base64::new(alphabet);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(base64::ParseAlphabetError::DuplicatedByte(b'F')));
}

#[test]
fn test_new_boundary_conditions() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYYZabcde"; // 64 bytes with a duplicate
    let result = base64::new(alphabet);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(base64::ParseAlphabetError::DuplicatedByte(b'Y')));
}

