pub(crate) fn complete_quads_len(
    input: &[u8],
    input_len_rem: usize,
    output_len: usize,
    decode_table: &[u8; 256],
) -> Result<usize, DecodeSliceError> {
    debug_assert!(input.len() % 4 == input_len_rem);

    // detect a trailing invalid byte, like a newline, as a user convenience
    if input_len_rem == 1 {
        let last_byte = input[input.len() - 1];
        // exclude pad bytes; might be part of padding that extends from earlier in the input
        if last_byte != PAD_BYTE && decode_table[usize::from(last_byte)] == INVALID_VALUE {
            return Err(DecodeError::InvalidByte(input.len() - 1, last_byte).into());
        }
    };

    // skip last quad, even if it's complete, as it may have padding
    let input_complete_nonterminal_quads_len = input
        .len()
        .saturating_sub(input_len_rem)
        // if rem was 0, subtract 4 to avoid padding
        .saturating_sub(usize::from(input_len_rem == 0) * 4);
    debug_assert!(
        input.is_empty() || (1..=4).contains(&(input.len() - input_complete_nonterminal_quads_len))
    );

    // check that everything except the last quad handled by decode_suffix will fit
    if output_len < input_complete_nonterminal_quads_len / 4 * 3 {
        return Err(DecodeSliceError::OutputSliceTooSmall);
    };
    Ok(input_complete_nonterminal_quads_len)
}