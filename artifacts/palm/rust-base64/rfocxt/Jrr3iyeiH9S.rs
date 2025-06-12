use crate::{
    engine::{
        general_purpose::INVALID_VALUE, DecodeEstimate, DecodeMetadata, DecodePaddingMode,
    },
    DecodeError, DecodeSliceError, PAD_BYTE,
};
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
#[inline(always)]
fn decode_chunk_8(
    input: &[u8],
    index_at_start_of_input: usize,
    decode_table: &[u8; 256],
    output: &mut [u8],
) -> Result<(), DecodeError> {
    let morsel = decode_table[usize::from(input[0])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input, input[0]));
    }
    let mut accum = u64::from(morsel) << 58;
    let morsel = decode_table[usize::from(input[1])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input + 1, input[1]));
    }
    accum |= u64::from(morsel) << 52;
    let morsel = decode_table[usize::from(input[2])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input + 2, input[2]));
    }
    accum |= u64::from(morsel) << 46;
    let morsel = decode_table[usize::from(input[3])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input + 3, input[3]));
    }
    accum |= u64::from(morsel) << 40;
    let morsel = decode_table[usize::from(input[4])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input + 4, input[4]));
    }
    accum |= u64::from(morsel) << 34;
    let morsel = decode_table[usize::from(input[5])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input + 5, input[5]));
    }
    accum |= u64::from(morsel) << 28;
    let morsel = decode_table[usize::from(input[6])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input + 6, input[6]));
    }
    accum |= u64::from(morsel) << 22;
    let morsel = decode_table[usize::from(input[7])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input + 7, input[7]));
    }
    accum |= u64::from(morsel) << 16;
    output[..6].copy_from_slice(&accum.to_be_bytes()[..6]);
    Ok(())
}
