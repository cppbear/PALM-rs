// Answer 0

#[test]
fn test_fmt_invalid_length() {
    let error = ParseAlphabetError::InvalidLength;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid length - must be 64 bytes");
}

#[test]
fn test_fmt_duplicated_byte() {
    let error = ParseAlphabetError::DuplicatedByte(0x41); // 'A'
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Duplicated byte: 0x41");
}

#[test]
fn test_fmt_unprintable_byte() {
    let error = ParseAlphabetError::UnprintableByte(0x01); // unprintable byte
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Unprintable byte: 0x01");
}

#[test]
fn test_fmt_reserved_byte() {
    let error = ParseAlphabetError::ReservedByte(b'=');
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Reserved byte: 0x3d");
}

