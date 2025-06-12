// Answer 0

#[test]
fn test_try_from_valid_alphabet() {
    let valid_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::try_from(valid_alphabet);
    assert!(result.is_ok());
    let alphabet = result.unwrap();
    assert_eq!(alphabet.as_str(), valid_alphabet);
}

#[test]
fn test_try_from_invalid_length() {
    let invalid_length = "ABC"; // Length is less than 64
    let result = Alphabet::try_from(invalid_length);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_try_from_unprintable_byte() {
    let unprintable = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x00"; // Contains a null byte
    let result = Alphabet::try_from(unprintable);
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(0)));
}

#[test]
fn test_try_from_reserved_byte() {
    let reserved_byte = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+="; // Contains the PAD_BYTE
    let result = Alphabet::try_from(reserved_byte);
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(b'=')));
}

#[test]
fn test_try_from_duplicated_byte() {
    let duplicated = "ABCDEFABCDEGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // Contains duplicated 'A'
    let result = Alphabet::try_from(duplicated);
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'A')));
}

