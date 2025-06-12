// Answer 0

#[derive(Debug)]
enum Error {
    InvalidLength,
    DuplicatedByte(u8),
    UnprintableByte(u8),
    ReservedByte(u8),
}

impl std::fmt::Display for Error {
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
    let error = Error::InvalidLength;
    let result = format!("{}", error);
    assert_eq!(result, "Invalid length - must be 64 bytes");
}

#[test]
fn test_duplicated_byte() {
    let error = Error::DuplicatedByte(0xAB);
    let result = format!("{}", error);
    assert_eq!(result, "Duplicated byte: 0xab");
}

#[test]
fn test_unprintable_byte() {
    let error = Error::UnprintableByte(0x1F);
    let result = format!("{}", error);
    assert_eq!(result, "Unprintable byte: 0x1f");
}

#[test]
fn test_reserved_byte() {
    let error = Error::ReservedByte(0x7F);
    let result = format!("{}", error);
    assert_eq!(result, "Reserved byte: 0x7f");
}

