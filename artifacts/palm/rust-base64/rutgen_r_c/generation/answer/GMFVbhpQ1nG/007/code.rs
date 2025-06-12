// Answer 0

#[test]
fn test_new_alphabet_with_unprintable_byte() {
    // This input includes a byte that is unprintable (byte 127)
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/A";
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(127)));
}

#[test]
fn test_new_alphabet_with_duplicate_byte() {
    // This input includes a duplicated byte (two 'A's)
    let alphabet = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'A')));
}

#[test]
fn test_new_alphabet_with_invalid_length() {
    // This input is shorter than 64 bytes, which is invalid
    let alphabet = "ABCDEF";
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_alphabet_with_reserved_byte() {
    // This input includes a reserved byte (the padding byte '=')
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+=";
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(b'=')));
}

#[test]
fn test_new_alphabet_with_unique_printable_bytes() {
    // This input includes all unique printable bytes within the allowed range
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::new(alphabet);
    assert!(result.is_ok());
}

