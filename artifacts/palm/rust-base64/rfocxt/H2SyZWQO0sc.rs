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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Alphabet {
    pub(crate) symbols: [u8; ALPHABET_SIZE],
}
pub(crate) const fn decode_table(alphabet: &Alphabet) -> [u8; 256] {
    let mut decode_table = [INVALID_VALUE; 256];
    let mut index = 0;
    while index < 64 {
        decode_table[alphabet.symbols[index] as usize] = index as u8;
        index += 1;
    }
    decode_table
}
