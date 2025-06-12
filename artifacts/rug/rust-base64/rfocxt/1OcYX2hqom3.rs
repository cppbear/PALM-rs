use crate::{
    engine::{
        general_purpose::INVALID_VALUE, DecodeEstimate, DecodeMetadata, DecodePaddingMode,
    },
    DecodeError, DecodeSliceError, PAD_BYTE,
};
pub struct GeneralPurposeEstimate {
    /// input len % 4
    rem: usize,
    conservative_decoded_len: usize,
}
#[derive(PartialEq, Eq, Debug)]
pub struct DecodeMetadata {
    /// Number of decoded bytes output
    pub(crate) decoded_len: usize,
    /// Offset of the first padding byte in the input, if any
    pub(crate) padding_offset: Option<usize>,
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeSliceError {
    /// A [`DecodeError`] occurred
    DecodeError(DecodeError),
    /// The provided slice is too small.
    OutputSliceTooSmall,
}
#[inline]
pub(crate) fn decode_helper(
    input: &[u8],
    estimate: &GeneralPurposeEstimate,
    output: &mut [u8],
    decode_table: &[u8; 256],
    decode_allow_trailing_bits: bool,
    padding_mode: DecodePaddingMode,
) -> Result<DecodeMetadata, DecodeSliceError> {
    let input_complete_nonterminal_quads_len = complete_quads_len(
        input,
        estimate.rem,
        output.len(),
        decode_table,
    )?;
    const UNROLLED_INPUT_CHUNK_SIZE: usize = 32;
    const UNROLLED_OUTPUT_CHUNK_SIZE: usize = UNROLLED_INPUT_CHUNK_SIZE / 4 * 3;
    let input_complete_quads_after_unrolled_chunks_len = input_complete_nonterminal_quads_len
        % UNROLLED_INPUT_CHUNK_SIZE;
    let input_unrolled_loop_len = input_complete_nonterminal_quads_len
        - input_complete_quads_after_unrolled_chunks_len;
    for (chunk_index, chunk) in input[..input_unrolled_loop_len]
        .chunks_exact(UNROLLED_INPUT_CHUNK_SIZE)
        .enumerate()
    {
        let input_index = chunk_index * UNROLLED_INPUT_CHUNK_SIZE;
        let chunk_output = &mut output[chunk_index
            * UNROLLED_OUTPUT_CHUNK_SIZE..(chunk_index + 1)
            * UNROLLED_OUTPUT_CHUNK_SIZE];
        decode_chunk_8(
            &chunk[0..8],
            input_index,
            decode_table,
            &mut chunk_output[0..6],
        )?;
        decode_chunk_8(
            &chunk[8..16],
            input_index + 8,
            decode_table,
            &mut chunk_output[6..12],
        )?;
        decode_chunk_8(
            &chunk[16..24],
            input_index + 16,
            decode_table,
            &mut chunk_output[12..18],
        )?;
        decode_chunk_8(
            &chunk[24..32],
            input_index + 24,
            decode_table,
            &mut chunk_output[18..24],
        )?;
    }
    let output_unrolled_loop_len = input_unrolled_loop_len / 4 * 3;
    let output_complete_quad_len = input_complete_nonterminal_quads_len / 4 * 3;
    {
        let output_after_unroll = &mut output[output_unrolled_loop_len..output_complete_quad_len];
        for (chunk_index, chunk) in input[input_unrolled_loop_len..input_complete_nonterminal_quads_len]
            .chunks_exact(4)
            .enumerate()
        {
            let chunk_output = &mut output_after_unroll[chunk_index
                * 3..chunk_index * 3 + 3];
            decode_chunk_4(
                chunk,
                input_unrolled_loop_len + chunk_index * 4,
                decode_table,
                chunk_output,
            )?;
        }
    }
    super::decode_suffix::decode_suffix(
        input,
        input_complete_nonterminal_quads_len,
        output,
        output_complete_quad_len,
        decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    )
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
#[inline(always)]
fn decode_chunk_4(
    input: &[u8],
    index_at_start_of_input: usize,
    decode_table: &[u8; 256],
    output: &mut [u8],
) -> Result<(), DecodeError> {
    let morsel = decode_table[usize::from(input[0])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input, input[0]));
    }
    let mut accum = u32::from(morsel) << 26;
    let morsel = decode_table[usize::from(input[1])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input + 1, input[1]));
    }
    accum |= u32::from(morsel) << 20;
    let morsel = decode_table[usize::from(input[2])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input + 2, input[2]));
    }
    accum |= u32::from(morsel) << 14;
    let morsel = decode_table[usize::from(input[3])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input + 3, input[3]));
    }
    accum |= u32::from(morsel) << 8;
    output[..3].copy_from_slice(&accum.to_be_bytes()[..3]);
    Ok(())
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
