//! Implementations of `io::Write` to transparently handle base64.
mod encoder;
mod encoder_string_writer;

pub use self::{
    encoder::EncoderWriter,
    encoder_string_writer::{EncoderStringWriter, StrConsumer},
};

#[cfg(test)]
mod encoder_tests;

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use engine::Config;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
	use std::convert::From;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_11() {
    rusty_monitor::set_test_id(11);
    let mut u8_0: u8 = 17u8;
    let mut usize_0: usize = 3982usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_0);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut usize_1: usize = 4521usize;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_1);
    let mut encodesliceerror_1: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_1_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_1;
    let mut bool_0: bool = true;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut u8_1: u8 = 82u8;
    let mut usize_2: usize = 8940usize;
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_2, u8_1);
    let mut decodeerror_3_ref_0: &decode::DecodeError = &mut decodeerror_3;
    let mut decodeerror_4: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_4_ref_0: &decode::DecodeError = &mut decodeerror_4;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut decodeerror_5: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut bool_1: bool = true;
    let mut usize_3: usize = 2298usize;
    let mut option_0: std::option::Option<usize> = crate::encode::encoded_len(usize_3, bool_1);
    let mut usize_4: usize = std::option::Option::unwrap(option_0);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_5);
    let mut tuple_0: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_0_ref_0);
    let mut bool_2: bool = crate::decode::DecodeError::eq(decodeerror_4_ref_0, decodeerror_3_ref_0);
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_allow_trailing_bits(generalpurposeconfig_0, bool_0);
    let mut tuple_1: () = crate::encode::EncodeSliceError::assert_receiver_is_total_eq(encodesliceerror_1_ref_0);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_2);
    let mut tuple_2: () = crate::encode::EncodeSliceError::assert_receiver_is_total_eq(encodesliceerror_0_ref_0);
    let mut bool_3: bool = crate::decode::DecodeError::eq(decodeerror_1_ref_0, decodeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_encode_padding(generalpurposeconfig_0, bool_2);
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut u8_0: u8 = 84u8;
    let mut parsealphabeterror_1: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::DuplicatedByte(u8_0);
    let mut parsealphabeterror_1_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_1;
    let mut usize_0: usize = 1738usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut usize_1: usize = 2800usize;
    let mut bool_3: bool = false;
    let mut bool_4: bool = true;
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut generalpurposeconfig_3: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_encode_padding(generalpurposeconfig_2, bool_4);
    let mut generalpurposeconfig_4: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_allow_trailing_bits(generalpurposeconfig_3, bool_3);
    let mut generalpurposeconfig_4_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_4;
    let mut generalpurposeconfig_5: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_4_ref_0);
    let mut generalpurposeconfig_5_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_5;
    let mut bool_5: bool = crate::engine::general_purpose::GeneralPurposeConfig::encode_padding(generalpurposeconfig_5_ref_0);
    let mut bool_6: bool = crate::alphabet::ParseAlphabetError::eq(parsealphabeterror_1_ref_0, parsealphabeterror_0_ref_0);
    let mut tuple_0: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_0_ref_0);
    let mut generalpurposeconfig_6: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_allow_trailing_bits(generalpurposeconfig_1, bool_1);
    let mut generalpurposeconfig_7: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_encode_padding(generalpurposeconfig_6, bool_0);
    let mut generalpurposeconfig_7_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_7;
    panic!("From RustyUnit with love");
}
}