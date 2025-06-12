// Answer 0

#[derive(Debug)]
enum MyError {
    InvalidLength,
    DuplicatedByte(u8),
    UnprintableByte(u8),
    ReservedByte(u8),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "Invalid length - must be 64 bytes"),
            Self::DuplicatedByte(b) => write!(f, "Duplicated byte: {:#04x}", b),
            Self::UnprintableByte(b) => write!(f, "Unprintable byte: {:#04x}", b),
            Self::ReservedByte(b) => write!(f, "Reserved byte: {:#04x}", b),
        }
    }
}

#[test]
fn test_invalid_length() {
    let error = MyError::InvalidLength;
    let result = format!("{}", error);
    assert_eq!(result, "Invalid length - must be 64 bytes");
}

#[test]
fn test_duplicated_byte() {
    let error = MyError::DuplicatedByte(0xAB);
    let result = format!("{}", error);
    assert_eq!(result, "Duplicated byte: 0xab");
}

#[test]
fn test_unprintable_byte() {
    let error = MyError::UnprintableByte(0x01); // Assuming 0x01 is unprintable
    let result = format!("{}", error);
    assert_eq!(result, "Unprintable byte: 0x01");
}

#[test]
fn test_reserved_byte() {
    let error = MyError::ReservedByte(0xFF); // Assuming 0xFF is a reserved byte
    let result = format!("{}", error);
    assert_eq!(result, "Reserved byte: 0xff");
}

