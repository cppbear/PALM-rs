// Answer 0

#[test]
fn test_new_invalid_length() {
    let result = Alphabet::new("short");
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_unprintable_byte() {
    let unprintable_char = 31_u8 as char; // Unprintable ASCII character
    let alphabet = format!("ABCDEFGHJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/{unprintable_char}");
    let result = Alphabet::new(&alphabet);
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(unprintable_char as u8)));
}

#[test]
fn test_new_duplicate_byte() {
    let alphabet = "ABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCD"; // 'A' is duplicated
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'A')));
}

#[test]
fn test_new_reserved_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+="; // '=' is used
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(b'=')));
}

