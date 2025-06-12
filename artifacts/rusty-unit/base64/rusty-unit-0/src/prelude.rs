//! Preconfigured engines for common use cases.
//!
//! These are re-exports of `const` engines in [crate::engine::general_purpose], renamed with a `BASE64_`
//! prefix for those who prefer to `use` the entire path to a name.
//!
//! # Examples
//!
#![cfg_attr(feature = "alloc", doc = "```")]
#![cfg_attr(not(feature = "alloc"), doc = "```ignore")]
//! use base64::prelude::{Engine as _, BASE64_STANDARD_NO_PAD};
//!
//! assert_eq!("c29tZSBieXRlcw", &BASE64_STANDARD_NO_PAD.encode(b"some bytes"));
//! ```

pub use crate::engine::Engine;

pub use crate::engine::general_purpose::STANDARD as BASE64_STANDARD;
pub use crate::engine::general_purpose::STANDARD_NO_PAD as BASE64_STANDARD_NO_PAD;
pub use crate::engine::general_purpose::URL_SAFE as BASE64_URL_SAFE;
pub use crate::engine::general_purpose::URL_SAFE_NO_PAD as BASE64_URL_SAFE_NO_PAD;

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::error::Error;
	use std::convert::From;
	use std::cmp::Eq;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_21() {
    rusty_monitor::set_test_id(21);
    let mut u8_0: u8 = 99u8;
    let mut usize_0: usize = 6955usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_0, u8_0);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_0);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut u8_1: u8 = 111u8;
    let mut usize_1: usize = 5128usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_1, u8_1);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_1);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut u8_2: u8 = 35u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::ReservedByte(u8_2);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut u8_3: u8 = 78u8;
    let mut parsealphabeterror_1: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::ReservedByte(u8_3);
    let mut parsealphabeterror_1_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_1;
    let mut usize_2: usize = 2783usize;
    let mut encodesliceerror_1: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_1_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_1;
    let mut encodesliceerror_2: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_2_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_2;
    let mut decodesliceerror_2: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_2_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_2;
    let mut encodesliceerror_3: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut option_0: std::option::Option<&dyn std::error::Error> = crate::decode::DecodeSliceError::source(decodesliceerror_2_ref_0);
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut bool_0: bool = crate::encode::EncodeSliceError::eq(encodesliceerror_2_ref_0, encodesliceerror_1_ref_0);
    let mut usize_3: usize = crate::decode::decoded_len_estimate(usize_2);
    let mut bool_1: bool = crate::alphabet::ParseAlphabetError::eq(parsealphabeterror_1_ref_0, parsealphabeterror_0_ref_0);
    let mut tuple_0: () = crate::encode::EncodeSliceError::assert_receiver_is_total_eq(encodesliceerror_0_ref_0);
    let mut bool_2: bool = crate::decode::DecodeSliceError::eq(decodesliceerror_1_ref_0, decodesliceerror_0_ref_0);
    panic!("From RustyUnit with love");
}
}