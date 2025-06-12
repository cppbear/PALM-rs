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
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 1,
            input[1],
        ));
    }
    accum |= u32::from(morsel) << 20;

    let morsel = decode_table[usize::from(input[2])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 2,
            input[2],
        ));
    }
    accum |= u32::from(morsel) << 14;

    let morsel = decode_table[usize::from(input[3])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 3,
            input[3],
        ));
    }
    accum |= u32::from(morsel) << 8;

    output[..3].copy_from_slice(&accum.to_be_bytes()[..3]);

    Ok(())
}