//! Enables base64'd output anywhere you might use a `Display` implementation, like a format string.
//!
//! ```
//! use base64::{display::Base64Display, engine::general_purpose::STANDARD};
//!
//! let data = vec![0x0, 0x1, 0x2, 0x3];
//! let wrapper = Base64Display::new(&data, &STANDARD);
//!
//! assert_eq!("base64: AAECAw==", format!("base64: {}", wrapper));
//! ```

use super::chunked_encoder::ChunkedEncoder;
use crate::engine::Engine;
use core::fmt::{Display, Formatter};
use core::{fmt, str};

/// A convenience wrapper for base64'ing bytes into a format string without heap allocation.
pub struct Base64Display<'a, 'e, E: Engine> {
    bytes: &'a [u8],
    chunked_encoder: ChunkedEncoder<'e, E>,
}

impl<'a, 'e, E: Engine> Base64Display<'a, 'e, E> {
    /// Create a `Base64Display` with the provided engine.
    pub fn new(bytes: &'a [u8], engine: &'e E) -> Base64Display<'a, 'e, E> {
        Base64Display {
            bytes,
            chunked_encoder: ChunkedEncoder::new(engine),
        }
    }
}

impl<'a, 'e, E: Engine> Display for Base64Display<'a, 'e, E> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        let mut sink = FormatterSink { f: formatter };
        self.chunked_encoder.encode(self.bytes, &mut sink)
    }
}

struct FormatterSink<'a, 'b: 'a> {
    f: &'a mut Formatter<'b>,
}

impl<'a, 'b: 'a> super::chunked_encoder::Sink for FormatterSink<'a, 'b> {
    type Error = fmt::Error;

    fn write_encoded_bytes(&mut self, encoded: &[u8]) -> Result<(), Self::Error> {
        // Avoid unsafe. If max performance is needed, write your own display wrapper that uses
        // unsafe here to gain about 10-15%.
        self.f
            .write_str(str::from_utf8(encoded).expect("base64 data was not utf8"))
    }
}

#[cfg(test)]
mod tests {
    use super::super::chunked_encoder::tests::{
        chunked_encode_matches_normal_encode_random, SinkTestHelper,
    };
    use super::*;
    use crate::engine::general_purpose::STANDARD;

    #[test]
    fn basic_display() {
        assert_eq!(
            "~$Zm9vYmFy#*",
            format!("~${}#*", Base64Display::new(b"foobar", &STANDARD))
        );
        assert_eq!(
            "~$Zm9vYmFyZg==#*",
            format!("~${}#*", Base64Display::new(b"foobarf", &STANDARD))
        );
    }

    #[test]
    fn display_encode_matches_normal_encode() {
        let helper = DisplaySinkTestHelper;
        chunked_encode_matches_normal_encode_random(&helper);
    }

    struct DisplaySinkTestHelper;

    impl SinkTestHelper for DisplaySinkTestHelper {
        fn encode_to_string<E: Engine>(&self, engine: &E, bytes: &[u8]) -> String {
            format!("{}", Base64Display::new(bytes, engine))
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::convert::TryFrom;
	use std::cmp::Eq;
	use std::convert::From;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut usize_0: usize = 3543usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut str_0: &str = "xz0KglTzLev2qNI";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u8_0: u8 = 5u8;
    let mut usize_1: usize = 3589usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_1, u8_0);
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_2_ref_0: &decode::DecodeError = &mut decodeerror_2;
    let mut bool_0: bool = false;
    let mut bool_1: bool = false;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_allow_trailing_bits(generalpurposeconfig_0, bool_1);
    let mut usize_2: usize = 8365usize;
    let mut u8_1: u8 = 122u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_1);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut usize_3: usize = crate::decode::decoded_len_estimate(usize_2);
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_allow_trailing_bits(generalpurposeconfig_1, bool_0);
    let mut bool_2: bool = crate::decode::DecodeError::eq(decodeerror_2_ref_0, decodeerror_1_ref_0);
    let mut result_0: std::result::Result<crate::alphabet::Alphabet, alphabet::ParseAlphabetError> = crate::alphabet::Alphabet::try_from(str_0_ref_0);
    let mut tuple_0: () = crate::encode::EncodeSliceError::assert_receiver_is_total_eq(encodesliceerror_0_ref_0);
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::clone(decodeerror_0_ref_0);
    let mut tuple_1: () = crate::decode::DecodeSliceError::assert_receiver_is_total_eq(decodesliceerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_1_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_1;
    let mut u8_0: u8 = 36u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_0);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut u8_1: u8 = 15u8;
    let mut parsealphabeterror_1: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_1);
    let mut parsealphabeterror_1_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_1;
    let mut u8_2: u8 = 58u8;
    let mut parsealphabeterror_2: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::ReservedByte(u8_2);
    let mut parsealphabeterror_2_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_2;
    let mut u8_3: u8 = 68u8;
    let mut usize_0: usize = 4575usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_3);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_0);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut u8_4: u8 = 73u8;
    let mut parsealphabeterror_3: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::ReservedByte(u8_4);
    let mut parsealphabeterror_3_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_3;
    let mut bool_0: bool = crate::decode::DecodeSliceError::eq(decodesliceerror_1_ref_0, decodesliceerror_0_ref_0);
    let mut decodepaddingmode_2: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut decodepaddingmode_3: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_2_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_2;
    let mut bool_1: bool = crate::alphabet::ParseAlphabetError::eq(parsealphabeterror_2_ref_0, parsealphabeterror_1_ref_0);
    let mut decodepaddingmode_3_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_3;
    let mut bool_2: bool = crate::engine::DecodePaddingMode::eq(decodepaddingmode_1_ref_0, decodepaddingmode_0_ref_0);
    panic!("From RustyUnit with love");
}
}