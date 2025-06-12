use crate::PAD_BYTE;
use core::{convert, fmt};
#[cfg(any(feature = "std", test))]
use std::error;
const ALPHABET_SIZE: usize = 64;
pub const STANDARD: Alphabet = Alphabet::from_str_unchecked(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
);
pub const URL_SAFE: Alphabet = Alphabet::from_str_unchecked(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
);
pub const CRYPT: Alphabet = Alphabet::from_str_unchecked(
    "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
);
pub const BCRYPT: Alphabet = Alphabet::from_str_unchecked(
    "./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
);
pub const IMAP_MUTF7: Alphabet = Alphabet::from_str_unchecked(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,",
);
pub const BIN_HEX: Alphabet = Alphabet::from_str_unchecked(
    "!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr",
);
#[derive(Debug, Eq, PartialEq)]
pub enum ParseAlphabetError {
    /// Alphabets must be 64 ASCII bytes
    InvalidLength,
    /// All bytes must be unique
    DuplicatedByte(u8),
    /// All bytes must be printable (in the range `[32, 126]`).
    UnprintableByte(u8),
    /// `=` cannot be used
    ReservedByte(u8),
}
impl fmt::Display for ParseAlphabetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "Invalid length - must be 64 bytes"),
            Self::DuplicatedByte(b) => write!(f, "Duplicated byte: {:#04x}", b),
            Self::UnprintableByte(b) => write!(f, "Unprintable byte: {:#04x}", b),
            Self::ReservedByte(b) => write!(f, "Reserved byte: {:#04x}", b),
        }
    }
}
