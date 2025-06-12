use crate::{
    engine::{general_purpose::INVALID_VALUE, DecodeMetadata, DecodePaddingMode},
    DecodeError, DecodeSliceError, PAD_BYTE,
};
#[derive(PartialEq, Eq, Debug)]
pub struct DecodeMetadata {
    /// Number of decoded bytes output
    pub(crate) decoded_len: usize,
    /// Offset of the first padding byte in the input, if any
    pub(crate) padding_offset: Option<usize>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeSliceError {
    /// A [`DecodeError`] occurred
    DecodeError(DecodeError),
    /// The provided slice is too small.
    OutputSliceTooSmall,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// An invalid byte was found in the input. The offset and offending byte are provided.
    ///
    /// Padding characters (`=`) interspersed in the encoded form are invalid, as they may only
    /// be present as the last 0-2 bytes of input.
    ///
    /// This error may also indicate that extraneous trailing input bytes are present, causing
    /// otherwise valid padding to no longer be the last bytes of input.
    InvalidByte(usize, u8),
    /// The length of the input, as measured in valid base64 symbols, is invalid.
    /// There must be 2-4 symbols in the last input quad.
    InvalidLength(usize),
    /// The last non-padding input symbol's encoded 6 bits have nonzero bits that will be discarded.
    /// This is indicative of corrupted or truncated Base64.
    /// Unlike [`DecodeError::InvalidByte`], which reports symbols that aren't in the alphabet,
    /// this error is for symbols that are in the alphabet but represent nonsensical encodings.
    InvalidLastSymbol(usize, u8),
    /// The nature of the padding was not as configured: absent or incorrect when it must be
    /// canonical, or present when it must be absent, etc.
    InvalidPadding,
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
impl DecodeMetadata {
    pub(crate) fn new(decoded_bytes: usize, padding_index: Option<usize>) -> Self {
        Self {
            decoded_len: decoded_bytes,
            padding_offset: padding_index,
        }
    }
}
pub(crate) fn decode_suffix(
    input: &[u8],
    input_index: usize,
    output: &mut [u8],
    mut output_index: usize,
    decode_table: &[u8; 256],
    decode_allow_trailing_bits: bool,
    padding_mode: DecodePaddingMode,
) -> Result<DecodeMetadata, DecodeSliceError> {
    debug_assert!((input.len() - input_index) <= 4);
    let mut morsels_in_leftover = 0;
    let mut padding_bytes_count = 0;
    let mut first_padding_offset: usize = 0;
    let mut last_symbol = 0_u8;
    let mut morsels = [0_u8; 4];
    for (leftover_index, &b) in input[input_index..].iter().enumerate() {
        if b == PAD_BYTE {
            if leftover_index < 2 {
                debug_assert!(
                    leftover_index == 0 || (leftover_index == 1 && padding_bytes_count ==
                    0)
                );
                let bad_padding_index = input_index + leftover_index;
                return Err(DecodeError::InvalidByte(bad_padding_index, b).into());
            }
            if padding_bytes_count == 0 {
                first_padding_offset = leftover_index;
            }
            padding_bytes_count += 1;
            continue;
        }
        if padding_bytes_count > 0 {
            return Err(
                DecodeError::InvalidByte(input_index + first_padding_offset, PAD_BYTE)
                    .into(),
            );
        }
        last_symbol = b;
        let morsel = decode_table[b as usize];
        if morsel == INVALID_VALUE {
            return Err(DecodeError::InvalidByte(input_index + leftover_index, b).into());
        }
        morsels[morsels_in_leftover] = morsel;
        morsels_in_leftover += 1;
    }
    if !input.is_empty() && morsels_in_leftover < 2 {
        return Err(DecodeError::InvalidLength(input_index + morsels_in_leftover).into());
    }
    match padding_mode {
        DecodePaddingMode::Indifferent => {}
        DecodePaddingMode::RequireCanonical => {
            if (padding_bytes_count + morsels_in_leftover) % 4 != 0 {
                return Err(DecodeError::InvalidPadding.into());
            }
        }
        DecodePaddingMode::RequireNone => {
            if padding_bytes_count > 0 {
                return Err(DecodeError::InvalidPadding.into());
            }
        }
    }
    let leftover_bytes_to_append = morsels_in_leftover * 6 / 8;
    let mut leftover_num = (u32::from(morsels[0]) << 26) | (u32::from(morsels[1]) << 20)
        | (u32::from(morsels[2]) << 14) | (u32::from(morsels[3]) << 8);
    let mask = !0_u32 >> (leftover_bytes_to_append * 8);
    if !decode_allow_trailing_bits && (leftover_num & mask) != 0 {
        return Err(
            DecodeError::InvalidLastSymbol(
                    input_index + morsels_in_leftover - 1,
                    last_symbol,
                )
                .into(),
        );
    }
    for _ in 0..leftover_bytes_to_append {
        let hi_byte = (leftover_num >> 24) as u8;
        leftover_num <<= 8;
        *output.get_mut(output_index).ok_or(DecodeSliceError::OutputSliceTooSmall)? = hi_byte;
        output_index += 1;
    }
    Ok(
        DecodeMetadata::new(
            output_index,
            if padding_bytes_count > 0 {
                Some(input_index + first_padding_offset)
            } else {
                None
            },
        ),
    )
}
