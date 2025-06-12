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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Alphabet {
    pub(crate) symbols: [u8; ALPHABET_SIZE],
}
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
impl convert::TryFrom<&str> for Alphabet {
    type Error = ParseAlphabetError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl Alphabet {
    const fn from_str_unchecked(alphabet: &str) -> Self {
        let mut symbols = [0_u8; ALPHABET_SIZE];
        let source_bytes = alphabet.as_bytes();
        let mut index = 0;
        while index < ALPHABET_SIZE {
            symbols[index] = source_bytes[index];
            index += 1;
        }
        Self { symbols }
    }
    pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
        let bytes = alphabet.as_bytes();
        if bytes.len() != ALPHABET_SIZE {
            return Err(ParseAlphabetError::InvalidLength);
        }
        {
            let mut index = 0;
            while index < ALPHABET_SIZE {
                let byte = bytes[index];
                if !(byte >= 32_u8 && byte <= 126_u8) {
                    return Err(ParseAlphabetError::UnprintableByte(byte));
                }
                if byte == PAD_BYTE {
                    return Err(ParseAlphabetError::ReservedByte(byte));
                }
                let mut probe_index = 0;
                while probe_index < ALPHABET_SIZE {
                    if probe_index == index {
                        probe_index += 1;
                        continue;
                    }
                    let probe_byte = bytes[probe_index];
                    if byte == probe_byte {
                        return Err(ParseAlphabetError::DuplicatedByte(byte));
                    }
                    probe_index += 1;
                }
                index += 1;
            }
        }
        Ok(Self::from_str_unchecked(alphabet))
    }
    #[must_use]
    pub fn as_str(&self) -> &str {}
}
