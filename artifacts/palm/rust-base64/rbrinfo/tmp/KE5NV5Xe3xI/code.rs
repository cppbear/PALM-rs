pub(crate) fn decode_helper(
    input: &[u8],
    estimate: &GeneralPurposeEstimate,
    output: &mut [u8],
    decode_table: &[u8; 256],
    decode_allow_trailing_bits: bool,
    padding_mode: DecodePaddingMode,
) -> Result<DecodeMetadata, DecodeSliceError> {
    let input_complete_nonterminal_quads_len =
        complete_quads_len(input, estimate.rem, output.len(), decode_table)?;

    const UNROLLED_INPUT_CHUNK_SIZE: usize = 32;
    const UNROLLED_OUTPUT_CHUNK_SIZE: usize = UNROLLED_INPUT_CHUNK_SIZE / 4 * 3;

    let input_complete_quads_after_unrolled_chunks_len =
        input_complete_nonterminal_quads_len % UNROLLED_INPUT_CHUNK_SIZE;

    let input_unrolled_loop_len =
        input_complete_nonterminal_quads_len - input_complete_quads_after_unrolled_chunks_len;

    // chunks of 32 bytes
    for (chunk_index, chunk) in input[..input_unrolled_loop_len]
        .chunks_exact(UNROLLED_INPUT_CHUNK_SIZE)
        .enumerate()
    {
        let input_index = chunk_index * UNROLLED_INPUT_CHUNK_SIZE;
        let chunk_output = &mut output[chunk_index * UNROLLED_OUTPUT_CHUNK_SIZE
            ..(chunk_index + 1) * UNROLLED_OUTPUT_CHUNK_SIZE];

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

    // remaining quads, except for the last possibly partial one, as it may have padding
    let output_unrolled_loop_len = input_unrolled_loop_len / 4 * 3;
    let output_complete_quad_len = input_complete_nonterminal_quads_len / 4 * 3;
    {
        let output_after_unroll = &mut output[output_unrolled_loop_len..output_complete_quad_len];

        for (chunk_index, chunk) in input
            [input_unrolled_loop_len..input_complete_nonterminal_quads_len]
            .chunks_exact(4)
            .enumerate()
        {
            let chunk_output = &mut output_after_unroll[chunk_index * 3..chunk_index * 3 + 3];

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