// Answer 0

#[test]
fn test_try_from_valid_standard_alphabet() {
    let result = Alphabet::try_from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    assert!(result.is_ok());
    let alphabet = result.unwrap();
    assert_eq!(alphabet.symbols, *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
}

#[test]
fn test_try_from_valid_url_safe_alphabet() {
    let result = Alphabet::try_from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");
    assert!(result.is_ok());
    let alphabet = result.unwrap();
    assert_eq!(alphabet.symbols, *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");
}

#[test]
fn test_try_from_invalid_length() {
    let result = Alphabet::try_from("ABC");
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_try_from_unprintable_byte() {
    let result = Alphabet::try_from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x7F");
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(0x7F)));
}

#[test]
fn test_try_from_reserved_byte() {
    let result = Alphabet::try_from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+=/");
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(b'=')));
}

#[test]
fn test_try_from_duplicated_byte() {
    let result = Alphabet::try_from("ABCDEFABCDEGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'A')));
}

