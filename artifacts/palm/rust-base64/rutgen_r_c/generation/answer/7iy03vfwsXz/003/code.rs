// Answer 0

#[test]
fn test_fmt_invalid_length() {
    let error = ParseAlphabetError::InvalidLength;
    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "Invalid length - must be 64 bytes");
}

#[test]
fn test_fmt_duplicated_byte() {
    let byte: u8 = 0x01;
    let error = ParseAlphabetError::DuplicatedByte(byte);
    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "Duplicated byte: 0x01");
}

#[test]
fn test_fmt_unprintable_byte() {
    let byte: u8 = 0x7f; // DEL - unprintable
    let error = ParseAlphabetError::UnprintableByte(byte);
    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "Unprintable byte: 0x7f");
}

#[test]
fn test_fmt_reserved_byte() {
    let byte: u8 = b'='; // '=' is reserved
    let error = ParseAlphabetError::ReservedByte(byte);
    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "Reserved byte: 0x3d");
}

