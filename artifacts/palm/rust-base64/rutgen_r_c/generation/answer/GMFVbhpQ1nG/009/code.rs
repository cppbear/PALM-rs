// Answer 0

#[test]
fn test_valid_alphabet() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::new(alphabet);
    assert!(result.is_ok());
}

#[test]
fn test_alphabet_with_invalid_length() {
    let alphabet = "ABCDEF";
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_alphabet_with_non_printable_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz01234567\x00/";
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(0)));
}

#[test]
fn test_alphabet_with_reserved_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+=";
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(61))); // ASCII for '='
}

#[test]
fn test_alphabet_with_duplicated_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let alphabet_with_dup = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+a";
    let result = Alphabet::new(alphabet_with_dup);
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'a')));
}

