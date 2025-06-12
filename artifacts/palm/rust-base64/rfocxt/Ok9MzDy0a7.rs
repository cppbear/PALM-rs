use crate::{
    alphabet, alphabet::Alphabet, engine::{Config, DecodeMetadata, DecodePaddingMode},
    DecodeSliceError,
};
use core::convert::TryInto;
pub use decode::GeneralPurposeEstimate;
pub(crate) const INVALID_VALUE: u8 = 255;
pub const STANDARD: GeneralPurpose = GeneralPurpose::new(&alphabet::STANDARD, PAD);
pub const STANDARD_PAD_INDIFFERENT: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    PAD_INDIFFERENT,
);
pub const STANDARD_NO_PAD: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    NO_PAD,
);
pub const STANDARD_NO_PAD_INDIFFERENT: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    NO_PAD_INDIFFERENT,
);
pub const URL_SAFE: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, PAD);
pub const URL_SAFE_PAD_INDIFFERENT: GeneralPurpose = GeneralPurpose::new(
    &alphabet::URL_SAFE,
    PAD_INDIFFERENT,
);
pub const URL_SAFE_NO_PAD: GeneralPurpose = GeneralPurpose::new(
    &alphabet::URL_SAFE,
    NO_PAD,
);
pub const URL_SAFE_NO_PAD_INDIFFERENT: GeneralPurpose = GeneralPurpose::new(
    &alphabet::URL_SAFE,
    NO_PAD_INDIFFERENT,
);
pub const PAD: GeneralPurposeConfig = GeneralPurposeConfig::new();
pub const PAD_INDIFFERENT: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(true)
    .with_decode_padding_mode(DecodePaddingMode::Indifferent);
pub const NO_PAD: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(false)
    .with_decode_padding_mode(DecodePaddingMode::RequireNone);
pub const NO_PAD_INDIFFERENT: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(false)
    .with_decode_padding_mode(DecodePaddingMode::Indifferent);
#[inline]
fn read_u64(s: &[u8]) -> u64 {
    u64::from_be_bytes(s[..8].try_into().unwrap())
}
