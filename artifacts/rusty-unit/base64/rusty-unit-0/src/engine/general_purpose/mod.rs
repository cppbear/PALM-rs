//! Provides the [GeneralPurpose] engine and associated config types.
use crate::{
    alphabet,
    alphabet::Alphabet,
    engine::{Config, DecodeMetadata, DecodePaddingMode},
    DecodeSliceError,
};
use core::convert::TryInto;

pub(crate) mod decode;
pub(crate) mod decode_suffix;

pub use decode::GeneralPurposeEstimate;

pub(crate) const INVALID_VALUE: u8 = 255;

/// A general-purpose base64 engine.
///
/// - It uses no vector CPU instructions, so it will work on any system.
/// - It is reasonably fast (~2-3GiB/s).
/// - It is not constant-time, though, so it is vulnerable to timing side-channel attacks. For loading cryptographic keys, etc, it is suggested to use the forthcoming constant-time implementation.

#[derive(Debug, Clone)]
pub struct GeneralPurpose {
    encode_table: [u8; 64],
    decode_table: [u8; 256],
    config: GeneralPurposeConfig,
}

impl GeneralPurpose {
    /// Create a `GeneralPurpose` engine from an [Alphabet].
    ///
    /// While not very expensive to initialize, ideally these should be cached
    /// if the engine will be used repeatedly.
    pub const fn new(alphabet: &Alphabet, config: GeneralPurposeConfig) -> Self {
        Self {
            encode_table: encode_table(alphabet),
            decode_table: decode_table(alphabet),
            config,
        }
    }
}

impl super::Engine for GeneralPurpose {
    type Config = GeneralPurposeConfig;
    type DecodeEstimate = GeneralPurposeEstimate;

    fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
        let mut input_index: usize = 0;

        const BLOCKS_PER_FAST_LOOP: usize = 4;
        const LOW_SIX_BITS: u64 = 0x3F;

        // we read 8 bytes at a time (u64) but only actually consume 6 of those bytes. Thus, we need
        // 2 trailing bytes to be available to read..
        let last_fast_index = input.len().saturating_sub(BLOCKS_PER_FAST_LOOP * 6 + 2);
        let mut output_index = 0;

        if last_fast_index > 0 {
            while input_index <= last_fast_index {
                // Major performance wins from letting the optimizer do the bounds check once, mostly
                // on the output side
                let input_chunk =
                    &input[input_index..(input_index + (BLOCKS_PER_FAST_LOOP * 6 + 2))];
                let output_chunk =
                    &mut output[output_index..(output_index + BLOCKS_PER_FAST_LOOP * 8)];

                // Hand-unrolling for 32 vs 16 or 8 bytes produces yields performance about equivalent
                // to unsafe pointer code on a Xeon E5-1650v3. 64 byte unrolling was slightly better for
                // large inputs but significantly worse for 50-byte input, unsurprisingly. I suspect
                // that it's a not uncommon use case to encode smallish chunks of data (e.g. a 64-byte
                // SHA-512 digest), so it would be nice if that fit in the unrolled loop at least once.
                // Plus, single-digit percentage performance differences might well be quite different
                // on different hardware.

                let input_u64 = read_u64(&input_chunk[0..]);

                output_chunk[0] = self.encode_table[((input_u64 >> 58) & LOW_SIX_BITS) as usize];
                output_chunk[1] = self.encode_table[((input_u64 >> 52) & LOW_SIX_BITS) as usize];
                output_chunk[2] = self.encode_table[((input_u64 >> 46) & LOW_SIX_BITS) as usize];
                output_chunk[3] = self.encode_table[((input_u64 >> 40) & LOW_SIX_BITS) as usize];
                output_chunk[4] = self.encode_table[((input_u64 >> 34) & LOW_SIX_BITS) as usize];
                output_chunk[5] = self.encode_table[((input_u64 >> 28) & LOW_SIX_BITS) as usize];
                output_chunk[6] = self.encode_table[((input_u64 >> 22) & LOW_SIX_BITS) as usize];
                output_chunk[7] = self.encode_table[((input_u64 >> 16) & LOW_SIX_BITS) as usize];

                let input_u64 = read_u64(&input_chunk[6..]);

                output_chunk[8] = self.encode_table[((input_u64 >> 58) & LOW_SIX_BITS) as usize];
                output_chunk[9] = self.encode_table[((input_u64 >> 52) & LOW_SIX_BITS) as usize];
                output_chunk[10] = self.encode_table[((input_u64 >> 46) & LOW_SIX_BITS) as usize];
                output_chunk[11] = self.encode_table[((input_u64 >> 40) & LOW_SIX_BITS) as usize];
                output_chunk[12] = self.encode_table[((input_u64 >> 34) & LOW_SIX_BITS) as usize];
                output_chunk[13] = self.encode_table[((input_u64 >> 28) & LOW_SIX_BITS) as usize];
                output_chunk[14] = self.encode_table[((input_u64 >> 22) & LOW_SIX_BITS) as usize];
                output_chunk[15] = self.encode_table[((input_u64 >> 16) & LOW_SIX_BITS) as usize];

                let input_u64 = read_u64(&input_chunk[12..]);

                output_chunk[16] = self.encode_table[((input_u64 >> 58) & LOW_SIX_BITS) as usize];
                output_chunk[17] = self.encode_table[((input_u64 >> 52) & LOW_SIX_BITS) as usize];
                output_chunk[18] = self.encode_table[((input_u64 >> 46) & LOW_SIX_BITS) as usize];
                output_chunk[19] = self.encode_table[((input_u64 >> 40) & LOW_SIX_BITS) as usize];
                output_chunk[20] = self.encode_table[((input_u64 >> 34) & LOW_SIX_BITS) as usize];
                output_chunk[21] = self.encode_table[((input_u64 >> 28) & LOW_SIX_BITS) as usize];
                output_chunk[22] = self.encode_table[((input_u64 >> 22) & LOW_SIX_BITS) as usize];
                output_chunk[23] = self.encode_table[((input_u64 >> 16) & LOW_SIX_BITS) as usize];

                let input_u64 = read_u64(&input_chunk[18..]);

                output_chunk[24] = self.encode_table[((input_u64 >> 58) & LOW_SIX_BITS) as usize];
                output_chunk[25] = self.encode_table[((input_u64 >> 52) & LOW_SIX_BITS) as usize];
                output_chunk[26] = self.encode_table[((input_u64 >> 46) & LOW_SIX_BITS) as usize];
                output_chunk[27] = self.encode_table[((input_u64 >> 40) & LOW_SIX_BITS) as usize];
                output_chunk[28] = self.encode_table[((input_u64 >> 34) & LOW_SIX_BITS) as usize];
                output_chunk[29] = self.encode_table[((input_u64 >> 28) & LOW_SIX_BITS) as usize];
                output_chunk[30] = self.encode_table[((input_u64 >> 22) & LOW_SIX_BITS) as usize];
                output_chunk[31] = self.encode_table[((input_u64 >> 16) & LOW_SIX_BITS) as usize];

                output_index += BLOCKS_PER_FAST_LOOP * 8;
                input_index += BLOCKS_PER_FAST_LOOP * 6;
            }
        }

        // Encode what's left after the fast loop.

        const LOW_SIX_BITS_U8: u8 = 0x3F;

        let rem = input.len() % 3;
        let start_of_rem = input.len() - rem;

        // start at the first index not handled by fast loop, which may be 0.

        while input_index < start_of_rem {
            let input_chunk = &input[input_index..(input_index + 3)];
            let output_chunk = &mut output[output_index..(output_index + 4)];

            output_chunk[0] = self.encode_table[(input_chunk[0] >> 2) as usize];
            output_chunk[1] = self.encode_table
                [((input_chunk[0] << 4 | input_chunk[1] >> 4) & LOW_SIX_BITS_U8) as usize];
            output_chunk[2] = self.encode_table
                [((input_chunk[1] << 2 | input_chunk[2] >> 6) & LOW_SIX_BITS_U8) as usize];
            output_chunk[3] = self.encode_table[(input_chunk[2] & LOW_SIX_BITS_U8) as usize];

            input_index += 3;
            output_index += 4;
        }

        if rem == 2 {
            output[output_index] = self.encode_table[(input[start_of_rem] >> 2) as usize];
            output[output_index + 1] =
                self.encode_table[((input[start_of_rem] << 4 | input[start_of_rem + 1] >> 4)
                    & LOW_SIX_BITS_U8) as usize];
            output[output_index + 2] =
                self.encode_table[((input[start_of_rem + 1] << 2) & LOW_SIX_BITS_U8) as usize];
            output_index += 3;
        } else if rem == 1 {
            output[output_index] = self.encode_table[(input[start_of_rem] >> 2) as usize];
            output[output_index + 1] =
                self.encode_table[((input[start_of_rem] << 4) & LOW_SIX_BITS_U8) as usize];
            output_index += 2;
        }

        output_index
    }

    fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
        GeneralPurposeEstimate::new(input_len)
    }

    fn internal_decode(
        &self,
        input: &[u8],
        output: &mut [u8],
        estimate: Self::DecodeEstimate,
    ) -> Result<DecodeMetadata, DecodeSliceError> {
        decode::decode_helper(
            input,
            estimate,
            output,
            &self.decode_table,
            self.config.decode_allow_trailing_bits,
            self.config.decode_padding_mode,
        )
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }
}

/// Returns a table mapping a 6-bit index to the ASCII byte encoding of the index
pub(crate) const fn encode_table(alphabet: &Alphabet) -> [u8; 64] {
    // the encode table is just the alphabet:
    // 6-bit index lookup -> printable byte
    let mut encode_table = [0_u8; 64];
    {
        let mut index = 0;
        while index < 64 {
            encode_table[index] = alphabet.symbols[index];
            index += 1;
        }
    }

    encode_table
}

/// Returns a table mapping base64 bytes as the lookup index to either:
/// - [INVALID_VALUE] for bytes that aren't members of the alphabet
/// - a byte whose lower 6 bits are the value that was encoded into the index byte
pub(crate) const fn decode_table(alphabet: &Alphabet) -> [u8; 256] {
    let mut decode_table = [INVALID_VALUE; 256];

    // Since the table is full of `INVALID_VALUE` already, we only need to overwrite
    // the parts that are valid.
    let mut index = 0;
    while index < 64 {
        // The index in the alphabet is the 6-bit value we care about.
        // Since the index is in 0-63, it is safe to cast to u8.
        decode_table[alphabet.symbols[index] as usize] = index as u8;
        index += 1;
    }

    decode_table
}

#[inline]
fn read_u64(s: &[u8]) -> u64 {
    u64::from_be_bytes(s[..8].try_into().unwrap())
}

/// Contains configuration parameters for base64 encoding and decoding.
///
/// ```
/// # use base64::engine::GeneralPurposeConfig;
/// let config = GeneralPurposeConfig::new()
///     .with_encode_padding(false);
///     // further customize using `.with_*` methods as needed
/// ```
///
/// The constants [PAD] and [NO_PAD] cover most use cases.
///
/// To specify the characters used, see [Alphabet].
#[derive(Clone, Copy, Debug)]
pub struct GeneralPurposeConfig {
    encode_padding: bool,
    decode_allow_trailing_bits: bool,
    decode_padding_mode: DecodePaddingMode,
}

impl GeneralPurposeConfig {
    /// Create a new config with `padding` = `true`, `decode_allow_trailing_bits` = `false`, and
    /// `decode_padding_mode = DecodePaddingMode::RequireCanonicalPadding`.
    ///
    /// This probably matches most people's expectations, but consider disabling padding to save
    /// a few bytes unless you specifically need it for compatibility with some legacy system.
    pub const fn new() -> Self {
        Self {
            // RFC states that padding must be applied by default
            encode_padding: true,
            decode_allow_trailing_bits: false,
            decode_padding_mode: DecodePaddingMode::RequireCanonical,
        }
    }

    /// Create a new config based on `self` with an updated `padding` setting.
    ///
    /// If `padding` is `true`, encoding will append either 1 or 2 `=` padding characters as needed
    /// to produce an output whose length is a multiple of 4.
    ///
    /// Padding is not needed for correct decoding and only serves to waste bytes, but it's in the
    /// [spec](https://datatracker.ietf.org/doc/html/rfc4648#section-3.2).
    ///
    /// For new applications, consider not using padding if the decoders you're using don't require
    /// padding to be present.
    pub const fn with_encode_padding(self, padding: bool) -> Self {
        Self {
            encode_padding: padding,
            ..self
        }
    }

    /// Create a new config based on `self` with an updated `decode_allow_trailing_bits` setting.
    ///
    /// Most users will not need to configure this. It's useful if you need to decode base64
    /// produced by a buggy encoder that has bits set in the unused space on the last base64
    /// character as per [forgiving-base64 decode](https://infra.spec.whatwg.org/#forgiving-base64-decode).
    /// If invalid trailing bits are present and this is `true`, those bits will
    /// be silently ignored, else `DecodeError::InvalidLastSymbol` will be emitted.
    pub const fn with_decode_allow_trailing_bits(self, allow: bool) -> Self {
        Self {
            decode_allow_trailing_bits: allow,
            ..self
        }
    }

    /// Create a new config based on `self` with an updated `decode_padding_mode` setting.
    ///
    /// Padding is not useful in terms of representing encoded data -- it makes no difference to
    /// the decoder if padding is present or not, so if you have some un-padded input to decode, it
    /// is perfectly fine to use `DecodePaddingMode::Indifferent` to prevent errors from being
    /// emitted.
    ///
    /// However, since in practice
    /// [people who learned nothing from BER vs DER seem to expect base64 to have one canonical encoding](https://eprint.iacr.org/2022/361),
    /// the default setting is the stricter `DecodePaddingMode::RequireCanonicalPadding`.
    ///
    /// Or, if "canonical" in your circumstance means _no_ padding rather than padding to the
    /// next multiple of four, there's `DecodePaddingMode::RequireNoPadding`.
    pub const fn with_decode_padding_mode(self, mode: DecodePaddingMode) -> Self {
        Self {
            decode_padding_mode: mode,
            ..self
        }
    }
}

impl Default for GeneralPurposeConfig {
    /// Delegates to [GeneralPurposeConfig::new].
    fn default() -> Self {
        Self::new()
    }
}

impl Config for GeneralPurposeConfig {
    fn encode_padding(&self) -> bool {
        self.encode_padding
    }
}

/// A [GeneralPurpose] engine using the [alphabet::STANDARD] base64 alphabet and [PAD] config.
pub const STANDARD: GeneralPurpose = GeneralPurpose::new(&alphabet::STANDARD, PAD);

/// A [GeneralPurpose] engine using the [alphabet::STANDARD] base64 alphabet and [NO_PAD] config.
pub const STANDARD_NO_PAD: GeneralPurpose = GeneralPurpose::new(&alphabet::STANDARD, NO_PAD);

/// A [GeneralPurpose] engine using the [alphabet::URL_SAFE] base64 alphabet and [PAD] config.
pub const URL_SAFE: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, PAD);

/// A [GeneralPurpose] engine using the [alphabet::URL_SAFE] base64 alphabet and [NO_PAD] config.
pub const URL_SAFE_NO_PAD: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, NO_PAD);

/// Include padding bytes when encoding, and require that they be present when decoding.
///
/// This is the standard per the base64 RFC, but consider using [NO_PAD] instead as padding serves
/// little purpose in practice.
pub const PAD: GeneralPurposeConfig = GeneralPurposeConfig::new();

/// Don't add padding when encoding, and require no padding when decoding.
pub const NO_PAD: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(false)
    .with_decode_padding_mode(DecodePaddingMode::RequireNone);

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use engine::Config;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::convert::From;
	use std::cmp::Eq;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut bool_0: bool = true;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut bool_1: bool = false;
    let mut bool_2: bool = true;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig {encode_padding: bool_2, decode_allow_trailing_bits: bool_1, decode_padding_mode: decodepaddingmode_1};
    let mut u8_0: u8 = 10u8;
    let mut usize_0: usize = 1365usize;
    let mut u8_1: u8 = 96u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_1);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut usize_1: usize = 1871usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_1);
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut u8_2: u8 = 98u8;
    let mut parsealphabeterror_1: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::ReservedByte(u8_2);
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::clone(decodeerror_1_ref_0);
    let mut decodeerror_2_ref_0: &decode::DecodeError = &mut decodeerror_2;
    let mut bool_3: bool = crate::decode::DecodeError::eq(decodeerror_2_ref_0, decodeerror_0_ref_0);
    let mut parsealphabeterror_2: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    let mut parsealphabeterror_1_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_1;
    let mut parsealphabeterror_2_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_2;
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_0);
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_allow_trailing_bits(generalpurposeconfig_0, bool_0);
    let mut decodepaddingmode_2: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::clone(decodepaddingmode_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut u8_0: u8 = 69u8;
    let mut usize_0: usize = 438usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_0);
    let mut usize_1: usize = 69usize;
    let mut bool_0: bool = true;
    let mut u8_1: u8 = 18u8;
    let mut usize_2: usize = 9388usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_2);
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut bool_1: bool = true;
    let mut bool_2: bool = false;
    let mut bool_3: bool = false;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_allow_trailing_bits(generalpurposeconfig_0, bool_3);
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_1_ref_0);
    let mut generalpurposeconfig_3: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig {encode_padding: bool_2, decode_allow_trailing_bits: bool_1, decode_padding_mode: decodepaddingmode_1};
    let mut generalpurposeconfig_4: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_padding_mode(generalpurposeconfig_2, decodepaddingmode_0);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_1);
    let mut decodepaddingmode_2: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_1);
    let mut encodesliceerror_1: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut generalpurposeconfig_5: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_allow_trailing_bits(generalpurposeconfig_4, bool_0);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_2: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_2: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10() {
    rusty_monitor::set_test_id(10);
    let mut u8_0: u8 = 102u8;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut usize_0: usize = 7944usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_0);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut u8_1: u8 = 52u8;
    let mut usize_1: usize = 7423usize;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut u8_2: u8 = 78u8;
    let mut usize_2: usize = 8189usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_2, u8_2);
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_2_ref_0: &decode::DecodeError = &mut decodeerror_2;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig {encode_padding: bool_1, decode_allow_trailing_bits: bool_0, decode_padding_mode: decodepaddingmode_1};
    let mut generalpurposeconfig_0_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_0;
    let mut bool_2: bool = crate::engine::general_purpose::GeneralPurposeConfig::encode_padding(generalpurposeconfig_0_ref_0);
    let mut tuple_0: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_0_ref_0);
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_1, u8_1);
    let mut decodeerror_3_ref_0: &decode::DecodeError = &mut decodeerror_3;
    let mut bool_3: bool = crate::decode::DecodeError::eq(decodeerror_3_ref_0, decodeerror_0_ref_0);
    let mut decodepaddingmode_2: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut tuple_1: () = crate::encode::EncodeSliceError::assert_receiver_is_total_eq(encodesliceerror_0_ref_0);
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_0);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut tuple_2: () = crate::alphabet::ParseAlphabetError::assert_receiver_is_total_eq(parsealphabeterror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut encodesliceerror_1: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_1_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_1;
    let mut u8_0: u8 = 67u8;
    let mut usize_0: usize = 4830usize;
    let mut u8_1: u8 = 11u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::ReservedByte(u8_1);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut bool_0: bool = false;
    let mut bool_1: bool = true;
    let mut u8_2: u8 = 94u8;
    let mut usize_1: usize = 1014usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_1, u8_2);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut u8_3: u8 = 119u8;
    let mut usize_2: usize = 9427usize;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_2, u8_3);
    let mut decodeerror_2_ref_0: &decode::DecodeError = &mut decodeerror_2;
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_3_ref_0: &decode::DecodeError = &mut decodeerror_3;
    let mut encodesliceerror_2: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut bool_2: bool = crate::decode::DecodeError::eq(decodeerror_3_ref_0, decodeerror_2_ref_0);
    let mut bool_3: bool = crate::decode::DecodeError::eq(decodeerror_1_ref_0, decodeerror_0_ref_0);
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig {encode_padding: bool_1, decode_allow_trailing_bits: bool_0, decode_padding_mode: decodepaddingmode_0};
    let mut decodeerror_4: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_0);
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut bool_4: bool = crate::encode::EncodeSliceError::eq(encodesliceerror_1_ref_0, encodesliceerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut u8_0: u8 = 103u8;
    let mut encodesliceerror_1: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_1_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_1;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut usize_0: usize = 5190usize;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut bool_0: bool = true;
    let mut bool_1: bool = false;
    let mut encodesliceerror_2: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_2_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_2;
    let mut decodepaddingmode_2: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_2_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_2;
    let mut encodesliceerror_3: encode::EncodeSliceError = crate::encode::EncodeSliceError::clone(encodesliceerror_2_ref_0);
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig {encode_padding: bool_1, decode_allow_trailing_bits: bool_0, decode_padding_mode: decodepaddingmode_1};
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_0);
    let mut decodepaddingmode_3: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut decodepaddingmode_4: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::clone(decodepaddingmode_0_ref_0);
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut decodepaddingmode_5: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_0);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::clone(decodesliceerror_0_ref_0);
    let mut decodepaddingmode_3_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_3;
    let mut tuple_0: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_3_ref_0);
    let mut tuple_1: () = crate::encode::EncodeSliceError::assert_receiver_is_total_eq(encodesliceerror_0_ref_0);
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_18() {
    rusty_monitor::set_test_id(18);
    let mut bool_0: bool = false;
    let mut usize_0: usize = 9198usize;
    let mut usize_1: usize = 7828usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_1);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_0);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut usize_2: usize = 7918usize;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_encode_padding(generalpurposeconfig_0, bool_2);
    let mut usize_3: usize = 2967usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_3);
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut bool_3: bool = false;
    let mut bool_4: bool = false;
    let mut u8_0: u8 = 33u8;
    let mut usize_4: usize = 7943usize;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_4, u8_0);
    let mut decodeerror_2_ref_0: &decode::DecodeError = &mut decodeerror_2;
    let mut tuple_0: () = crate::decode::DecodeError::assert_receiver_is_total_eq(decodeerror_2_ref_0);
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig {encode_padding: bool_4, decode_allow_trailing_bits: bool_3, decode_padding_mode: decodepaddingmode_0};
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::clone(decodeerror_1_ref_0);
    let mut generalpurposeconfig_3: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_encode_padding(generalpurposeconfig_1, bool_1);
    let mut generalpurposeconfig_4: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut decodeerror_4: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_2);
    let mut decodeerror_5: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut generalpurposeconfig_3_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_3;
    let mut generalpurposeconfig_5: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_3_ref_0);
    let mut option_0: std::option::Option<usize> = crate::encode::encoded_len(usize_0, bool_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut u8_0: u8 = 49u8;
    let mut usize_0: usize = 2805usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_0);
    let mut usize_1: usize = 5503usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_1);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_1);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut u8_1: u8 = 79u8;
    let mut usize_2: usize = 7155usize;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_2, u8_1);
    let mut decodesliceerror_2: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_2);
    let mut decodesliceerror_2_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_2;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut bool_0: bool = true;
    let mut bool_1: bool = true;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig {encode_padding: bool_1, decode_allow_trailing_bits: bool_0, decode_padding_mode: decodepaddingmode_1};
    let mut generalpurposeconfig_0_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_0;
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_0_ref_0);
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodepaddingmode_2: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::clone(decodepaddingmode_0_ref_0);
    let mut decodeerror_3_ref_0: &decode::DecodeError = &mut decodeerror_3;
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_1_ref_0);
    let mut bool_2: bool = crate::decode::DecodeSliceError::eq(decodesliceerror_2_ref_0, decodesliceerror_1_ref_0);
    let mut decodesliceerror_3: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut u8_0: u8 = 49u8;
    let mut usize_0: usize = 8496usize;
    let mut u8_1: u8 = 86u8;
    let mut usize_1: usize = 7788usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_1, u8_1);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_0);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut u8_2: u8 = 51u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::DuplicatedByte(u8_2);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut bool_0: bool = false;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut bool_1: bool = true;
    let mut bool_2: bool = true;
    let mut parsealphabeterror_1: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    let mut parsealphabeterror_1_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_1;
    let mut u8_3: u8 = 7u8;
    let mut usize_2: usize = 928usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_2, u8_3);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_1);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut tuple_0: () = crate::decode::DecodeSliceError::assert_receiver_is_total_eq(decodesliceerror_1_ref_0);
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig {encode_padding: bool_2, decode_allow_trailing_bits: bool_1, decode_padding_mode: decodepaddingmode_0};
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_encode_padding(generalpurposeconfig_0, bool_0);
    let mut tuple_1: () = crate::alphabet::ParseAlphabetError::assert_receiver_is_total_eq(parsealphabeterror_0_ref_0);
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut bool_3: bool = crate::engine::general_purpose::GeneralPurposeConfig::encode_padding(generalpurposeconfig_1_ref_0);
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_0, u8_0);
    let mut decodeerror_2_ref_0: &decode::DecodeError = &mut decodeerror_2;
    let mut tuple_2: () = crate::decode::DecodeError::assert_receiver_is_total_eq(decodeerror_2_ref_0);
    panic!("From RustyUnit with love");
}
}