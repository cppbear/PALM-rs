//! Provides [Alphabet] and constants for alphabets commonly used in the wild.

use crate::PAD_BYTE;
use core::{convert, fmt};
#[cfg(any(feature = "std", test))]
use std::error;

const ALPHABET_SIZE: usize = 64;

/// An alphabet defines the 64 ASCII characters (symbols) used for base64.
///
/// Common alphabets are provided as constants, and custom alphabets
/// can be made via `from_str` or the `TryFrom<str>` implementation.
///
/// # Examples
///
/// Building and using a custom Alphabet:
///
/// ```
/// let custom = base64::alphabet::Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/").unwrap();
///
/// let engine = base64::engine::GeneralPurpose::new(
///     &custom,
///     base64::engine::general_purpose::PAD);
/// ```
///
/// Building a const:
///
/// ```
/// use base64::alphabet::Alphabet;
///
/// static CUSTOM: Alphabet = {
///     // Result::unwrap() isn't const yet, but panic!() is OK
///     match Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/") {
///         Ok(x) => x,
///         Err(_) => panic!("creation of alphabet failed"),
///     }
/// };
/// ```
///
/// Building lazily:
///
/// ```
/// use base64::{
///     alphabet::Alphabet,
///     engine::{general_purpose::GeneralPurpose, GeneralPurposeConfig},
/// };
/// use once_cell::sync::Lazy;
///
/// static CUSTOM: Lazy<Alphabet> = Lazy::new(||
///     Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/").unwrap()
/// );
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Alphabet {
    pub(crate) symbols: [u8; ALPHABET_SIZE],
}

impl Alphabet {
    /// Performs no checks so that it can be const.
    /// Used only for known-valid strings.
    const fn from_str_unchecked(alphabet: &str) -> Self {
        let mut symbols = [0_u8; ALPHABET_SIZE];
        let source_bytes = alphabet.as_bytes();

        // a way to copy that's allowed in const fn
        let mut index = 0;
        while index < ALPHABET_SIZE {
            symbols[index] = source_bytes[index];
            index += 1;
        }

        Self { symbols }
    }

    /// Create an `Alphabet` from a string of 64 unique printable ASCII bytes.
    ///
    /// The `=` byte is not allowed as it is used for padding.
    pub const fn new(alphabet: &str) -> Result<Self, ParseAlphabetError> {
        let bytes = alphabet.as_bytes();
        if bytes.len() != ALPHABET_SIZE {
            return Err(ParseAlphabetError::InvalidLength);
        }

        {
            let mut index = 0;
            while index < ALPHABET_SIZE {
                let byte = bytes[index];

                // must be ascii printable. 127 (DEL) is commonly considered printable
                // for some reason but clearly unsuitable for base64.
                if !(byte >= 32_u8 && byte <= 126_u8) {
                    return Err(ParseAlphabetError::UnprintableByte(byte));
                }
                // = is assumed to be padding, so cannot be used as a symbol
                if byte == PAD_BYTE {
                    return Err(ParseAlphabetError::ReservedByte(byte));
                }

                // Check for duplicates while staying within what const allows.
                // It's n^2, but only over 64 hot bytes, and only once, so it's likely in the single digit
                // microsecond range.

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

    /// Create a `&str` from the symbols in the `Alphabet`
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.symbols).unwrap()
    }
}

impl convert::TryFrom<&str> for Alphabet {
    type Error = ParseAlphabetError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Possible errors when constructing an [Alphabet] from a `str`.
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

#[cfg(any(feature = "std", test))]
impl error::Error for ParseAlphabetError {}

/// The standard alphabet (with `+` and `/`) specified in [RFC 4648][].
///
/// [RFC 4648]: https://datatracker.ietf.org/doc/html/rfc4648#section-4
pub const STANDARD: Alphabet = Alphabet::from_str_unchecked(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
);

/// The URL-safe alphabet (with `-` and `_`) specified in [RFC 4648][].
///
/// [RFC 4648]: https://datatracker.ietf.org/doc/html/rfc4648#section-5
pub const URL_SAFE: Alphabet = Alphabet::from_str_unchecked(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
);

/// The `crypt(3)` alphabet (with `.` and `/` as the _first_ two characters).
///
/// Not standardized, but folk wisdom on the net asserts that this alphabet is what crypt uses.
pub const CRYPT: Alphabet = Alphabet::from_str_unchecked(
    "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
);

/// The bcrypt alphabet.
pub const BCRYPT: Alphabet = Alphabet::from_str_unchecked(
    "./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
);

/// The alphabet used in IMAP-modified UTF-7 (with `+` and `,`).
///
/// See [RFC 3501](https://tools.ietf.org/html/rfc3501#section-5.1.3)
pub const IMAP_MUTF7: Alphabet = Alphabet::from_str_unchecked(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+,",
);

/// The alphabet used in BinHex 4.0 files.
///
/// See [BinHex 4.0 Definition](http://files.stairways.com/other/binhex-40-specs-info.txt)
pub const BIN_HEX: Alphabet = Alphabet::from_str_unchecked(
    "!\"#$%&'()*+,-012345689@ABCDEFGHIJKLMNPQRSTUVXYZ[`abcdefhijklmpqr",
);

#[cfg(test)]
mod tests {
    use crate::alphabet::*;
    use core::convert::TryFrom as _;

    #[test]
    fn detects_duplicate_start() {
        assert_eq!(
            ParseAlphabetError::DuplicatedByte(b'A'),
            Alphabet::new("AACDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
                .unwrap_err()
        );
    }

    #[test]
    fn detects_duplicate_end() {
        assert_eq!(
            ParseAlphabetError::DuplicatedByte(b'/'),
            Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789//")
                .unwrap_err()
        );
    }

    #[test]
    fn detects_duplicate_middle() {
        assert_eq!(
            ParseAlphabetError::DuplicatedByte(b'Z'),
            Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZZbcdefghijklmnopqrstuvwxyz0123456789+/")
                .unwrap_err()
        );
    }

    #[test]
    fn detects_length() {
        assert_eq!(
            ParseAlphabetError::InvalidLength,
            Alphabet::new(
                "xxxxxxxxxABCDEFGHIJKLMNOPQRSTUVWXYZZbcdefghijklmnopqrstuvwxyz0123456789+/",
            )
            .unwrap_err()
        );
    }

    #[test]
    fn detects_padding() {
        assert_eq!(
            ParseAlphabetError::ReservedByte(b'='),
            Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+=")
                .unwrap_err()
        );
    }

    #[test]
    fn detects_unprintable() {
        // form feed
        assert_eq!(
            ParseAlphabetError::UnprintableByte(0xc),
            Alphabet::new("\x0cBCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
                .unwrap_err()
        );
    }

    #[test]
    fn same_as_unchecked() {
        assert_eq!(
            STANDARD,
            Alphabet::try_from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
                .unwrap()
        );
    }

    #[test]
    fn str_same_as_input() {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let a = Alphabet::try_from(alphabet).unwrap();
        assert_eq!(alphabet, a.as_str())
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::convert::TryFrom;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut u8_0: u8 = 59u8;
    let mut usize_0: usize = 2273usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_0, u8_0);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_0);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut usize_1: usize = 6047usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_1);
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut u8_1: u8 = 39u8;
    let mut usize_2: usize = 7228usize;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_2, u8_1);
    let mut decodeerror_2_ref_0: &decode::DecodeError = &mut decodeerror_2;
    let mut usize_3: usize = 6337usize;
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_padding_mode(generalpurposeconfig_0, decodepaddingmode_0);
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut result_0: std::result::Result<crate::alphabet::Alphabet, alphabet::ParseAlphabetError> = crate::alphabet::Alphabet::try_from(str_0_ref_0);
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_1_ref_0);
    let mut decodesliceerror_2: decode::DecodeSliceError = crate::decode::DecodeSliceError::clone(decodesliceerror_1_ref_0);
    let mut generalpurposeconfig_2_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_2;
    let mut generalpurposeconfig_3: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_2_ref_0);
    let mut bool_0: bool = crate::decode::DecodeError::eq(decodeerror_2_ref_0, decodeerror_1_ref_0);
    let mut decodesliceerror_2_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_2;
    let mut bool_1: bool = crate::decode::DecodeSliceError::eq(decodesliceerror_2_ref_0, decodesliceerror_0_ref_0);
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_encode_padding(generalpurposeconfig_0, bool_1);
    let mut u8_0: u8 = 20u8;
    let mut usize_0: usize = 9211usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_0, u8_0);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_0);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut encodesliceerror_1: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_1_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_1;
    let mut u8_1: u8 = 22u8;
    let mut usize_1: usize = 4042usize;
    let mut usize_2: usize = 2889usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_2);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_1);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut decodesliceerror_2: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_2_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_2;
    let mut u8_2: u8 = 118u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_2);
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodesliceerror_3: decode::DecodeSliceError = crate::decode::DecodeSliceError::clone(decodesliceerror_2_ref_0);
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodesliceerror_3_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_3;
    let mut bool_2: bool = crate::decode::DecodeSliceError::eq(decodesliceerror_3_ref_0, decodesliceerror_1_ref_0);
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_1, u8_1);
    let mut decodeerror_3_ref_0: &decode::DecodeError = &mut decodeerror_3;
    let mut bool_3: bool = crate::encode::EncodeSliceError::eq(encodesliceerror_1_ref_0, encodesliceerror_0_ref_0);
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_encode_padding(generalpurposeconfig_1, bool_0);
    panic!("From RustyUnit with love");
}
}