// Answer 0

#[test]
fn test_fmt_invalid_length() {
    let error = ParseAlphabetError::InvalidLength;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
    assert!(result.is_ok());
    assert_eq!(buffer, "Invalid length - must be 64 bytes");
}

#[test]
fn test_fmt_duplicated_byte() {
    let error = ParseAlphabetError::DuplicatedByte(0xFF);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
    assert!(result.is_ok());
    assert_eq!(buffer, "Duplicated byte: 0xff");
}

#[test]
fn test_fmt_unprintable_byte() {
    let error = ParseAlphabetError::UnprintableByte(0x1F);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
    assert!(result.is_ok());
    assert_eq!(buffer, "Unprintable byte: 0x1f");
}

#[test]
fn test_fmt_reserved_byte() {
    let error = ParseAlphabetError::ReservedByte(b'=');
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
    assert!(result.is_ok());
    assert_eq!(buffer, "Reserved byte: 0x3d");
}

