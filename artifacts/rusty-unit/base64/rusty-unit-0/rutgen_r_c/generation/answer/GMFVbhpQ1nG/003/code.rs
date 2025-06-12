// Answer 0

#[test]
fn test_new_alphabet_success() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_new_alphabet_invalid_length() {
    let result = Alphabet::new("Short");
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_alphabet_unprintable_byte() {
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefg\x01ijklmnopqrstuvwxyz0123456789+/");
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(1)));
}

#[test]
fn test_new_alphabet_reserved_byte() {
    const PAD_BYTE: u8 = b'='; // assuming PAD_BYTE is '='
    let result = Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz01234567+=");
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(PAD_BYTE)));
}

#[test]
fn test_new_alphabet_duplicated_byte() {
    let result = Alphabet::new("ABCDEFABCDEFABCDABCDABCDABCDABCDABCDABCDABCDABCDABCDABCDABCDABCD");
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'A')));
}

