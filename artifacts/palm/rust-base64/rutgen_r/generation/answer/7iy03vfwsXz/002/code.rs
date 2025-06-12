// Answer 0

#[test]
fn test_fmt_invalid_length() {
    #[derive(Debug)]
    enum ErrorType {
        InvalidLength,
        DuplicatedByte(u8),
        UnprintableByte(u8),
        ReservedByte(u8),
    }

    let error = ErrorType::InvalidLength;
    let result = format!("{}", error);
    assert_eq!(result, "Invalid length - must be 64 bytes");
}

#[test]
fn test_fmt_duplicated_byte() {
    #[derive(Debug)]
    enum ErrorType {
        InvalidLength,
        DuplicatedByte(u8),
        UnprintableByte(u8),
        ReservedByte(u8),
    }

    let error = ErrorType::DuplicatedByte(0xFF);
    let result = format!("{}", error);
    assert_eq!(result, "Duplicated byte: 0xff");
}

#[test]
fn test_fmt_unprintable_byte() {
    #[derive(Debug)]
    enum ErrorType {
        InvalidLength,
        DuplicatedByte(u8),
        UnprintableByte(u8),
        ReservedByte(u8),
    }

    let error = ErrorType::UnprintableByte(0x01);
    let result = format!("{}", error);
    assert_eq!(result, "Unprintable byte: 0x01");
}

#[test]
fn test_fmt_reserved_byte() {
    #[derive(Debug)]
    enum ErrorType {
        InvalidLength,
        DuplicatedByte(u8),
        UnprintableByte(u8),
        ReservedByte(u8),
    }

    let error = ErrorType::ReservedByte(0x7B);
    let result = format!("{}", error);
    assert_eq!(result, "Reserved byte: 0x7b");
}

