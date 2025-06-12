// Answer 0

use std::fmt;

enum Error {
    InvalidLength,
    DuplicatedByte(u8),
    UnprintableByte(u8),
    ReservedByte(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    let error = Error::DuplicatedByte(0x12);
    let result = format!("{}", error);
    assert_eq!(result, "Duplicated byte: 0x12");
}

#[test]
fn test_unprintable_byte() {
    let error = Error::UnprintableByte(0x01);
    let result = format!("{}", error);
    assert_eq!(result, "Unprintable byte: 0x01");
}

#[test]
fn test_reserved_byte() {
    let error = Error::ReservedByte(0xFF);
    let result = format!("{}", error);
    assert_eq!(result, "Reserved byte: 0xff");
}

