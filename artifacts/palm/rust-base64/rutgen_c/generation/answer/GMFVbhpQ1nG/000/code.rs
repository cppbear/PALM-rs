// Answer 0

#[test]
fn test_new_valid_alphabet() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; 
    let result = Alphabet::new(alphabet);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_new_invalid_length() {
    let alphabet = "ABC"; // Invalid length
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::InvalidLength));
}

#[test]
fn test_new_unprintable_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz01234567\x7F"; // DEL byte is unprintable
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::UnprintableByte(0x7F)));
}

#[test]
fn test_new_reserved_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+="; // '=' is reserved
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::ReservedByte(b'=')));
}

#[test]
fn test_new_duplicated_byte() {
    let alphabet = "ABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEF"; // Duplicate 'A'
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Err(ParseAlphabetError::DuplicatedByte(b'A')));
}

