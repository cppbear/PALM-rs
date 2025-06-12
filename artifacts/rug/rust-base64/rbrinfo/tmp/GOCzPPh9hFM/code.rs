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
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 1,
            input[1],
        ));
    }
    accum |= u64::from(morsel) << 52;

    let morsel = decode_table[usize::from(input[2])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 2,
            input[2],
        ));
    }
    accum |= u64::from(morsel) << 46;

    let morsel = decode_table[usize::from(input[3])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 3,
            input[3],
        ));
    }
    accum |= u64::from(morsel) << 40;

    let morsel = decode_table[usize::from(input[4])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 4,
            input[4],
        ));
    }
    accum |= u64::from(morsel) << 34;

    let morsel = decode_table[usize::from(input[5])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 5,
            input[5],
        ));
    }
    accum |= u64::from(morsel) << 28;

    let morsel = decode_table[usize::from(input[6])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 6,
            input[6],
        ));
    }
    accum |= u64::from(morsel) << 22;

    let morsel = decode_table[usize::from(input[7])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 7,
            input[7],
        ));
    }
    accum |= u64::from(morsel) << 16;

    output[..6].copy_from_slice(&accum.to_be_bytes()[..6]);

    Ok(())
}