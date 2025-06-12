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
pub trait Config {
    fn encode_padding(&self) -> bool;
}
#[derive(Clone, Copy, Debug)]
pub struct GeneralPurposeConfig {
    encode_padding: bool,
    decode_allow_trailing_bits: bool,
    decode_padding_mode: DecodePaddingMode,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodePaddingMode {
    /// Canonical padding is allowed, but any fewer padding bytes than that is also allowed.
    Indifferent,
    /// Padding must be canonical (0, 1, or 2 `=` as needed to produce a 4 byte suffix).
    RequireCanonical,
    /// Padding must be absent -- for when you want predictable padding, without any wasted bytes.
    RequireNone,
}
impl Config for GeneralPurposeConfig {
    fn encode_padding(&self) -> bool {
        self.encode_padding
    }
}
