use crate::{
    engine::{
        general_purpose::INVALID_VALUE, DecodeEstimate, DecodeMetadata, DecodePaddingMode,
    },
    DecodeError, DecodeSliceError, PAD_BYTE,
};
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
pub(crate) fn complete_quads_len(
    input: &[u8],
    input_len_rem: usize,
    output_len: usize,
    decode_table: &[u8; 256],
) -> Result<usize, DecodeSliceError> {
    debug_assert!(input.len() % 4 == input_len_rem);
    if input_len_rem == 1 {
        let last_byte = input[input.len() - 1];
        if last_byte != PAD_BYTE && decode_table[usize::from(last_byte)] == INVALID_VALUE
        {
            return Err(DecodeError::InvalidByte(input.len() - 1, last_byte).into());
        }
    }
    let input_complete_nonterminal_quads_len = input
        .len()
        .saturating_sub(input_len_rem)
        .saturating_sub(usize::from(input_len_rem == 0) * 4);
    debug_assert!(
        input.is_empty() || (1..= 4).contains(& (input.len() -
        input_complete_nonterminal_quads_len))
    );
    if output_len < input_complete_nonterminal_quads_len / 4 * 3 {
        return Err(DecodeSliceError::OutputSliceTooSmall);
    }
    Ok(input_complete_nonterminal_quads_len)
}
