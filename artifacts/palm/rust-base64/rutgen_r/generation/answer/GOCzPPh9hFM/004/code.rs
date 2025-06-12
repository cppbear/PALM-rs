// Answer 0

#[derive(Debug)]
struct DecodeError {
    index: usize,
    byte: u8,
}

const INVALID_VALUE: u8 = 255;

fn decode_chunk_8(
    input: &[u8],
    index_at_start_of_input: usize,
    decode_table: &[u8; 256],
    output: &mut [u8],
) -> Result<(), DecodeError> {
    let morsel = decode_table[usize::from(input[0])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError {
            index: index_at_start_of_input,
            byte: input[0],
        });
    }
    let mut accum = u64::from(morsel) << 58;

    let morsel = decode_table[usize::from(input[1])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError {
            index: index_at_start_of_input + 1,
            byte: input[1],
        });
    }
    accum |= u64::from(morsel) << 52;

    let morsel = decode_table[usize::from(input[2])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError {
            index: index_at_start_of_input + 2,
            byte: input[2],
        });
    }
    accum |= u64::from(morsel) << 46;

    let morsel = decode_table[usize::from(input[3])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError {
            index: index_at_start_of_input + 3,
            byte: input[3],
        });
    }
    accum |= u64::from(morsel) << 40;

    let morsel = decode_table[usize::from(input[4])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError {
            index: index_at_start_of_input + 4,
            byte: input[4],
        });
    }
    accum |= u64::from(morsel) << 34;

    let morsel = decode_table[usize::from(input[5])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError {
            index: index_at_start_of_input + 5,
            byte: input[5],
        });
    }
    accum |= u64::from(morsel) << 28;

    let morsel = decode_table[usize::from(input[6])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError {
            index: index_at_start_of_input + 6,
            byte: input[6],
        });
    }
    accum |= u64::from(morsel) << 22;

    let morsel = decode_table[usize::from(input[7])];
    if morsel == INVALID_VALUE {
        return Err(DecodeError {
            index: index_at_start_of_input + 7,
            byte: input[7],
        });
    }
    accum |= u64::from(morsel) << 16;

    output[..6].copy_from_slice(&accum.to_be_bytes()[..6]);

    Ok(())
}

#[test]
fn test_decode_chunk_8_valid_cases() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256]; // All invalid
    decode_table[b'A' as usize] = 0; // Example valid entry
    decode_table[b'B' as usize] = 1; // Example valid entry
    decode_table[b'C' as usize] = 2; // Example valid entry
    decode_table[b'D' as usize] = 3; // Example valid entry
    decode_table[b'E' as usize] = 4; // Example valid entry
    decode_table[b'F' as usize] = 5; // Example valid entry
    decode_table[b'G' as usize] = 6; // Example valid entry
    let mut output = [0; 6];

    assert!(decode_chunk_8(b"ABCDEFGH", 0, &decode_table, &mut output).is_ok());
    assert_eq!(&output[..], &1u8.to_be_bytes()[..6]); // Replace with actual expected output based on decode logic
}

#[test]
fn test_decode_chunk_8_invalid_first_case() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0;
    
    let mut output = [0; 6];
    assert_eq!(
        decode_chunk_8(b"ABCDEFXX", 0, &decode_table, &mut output),
        Err(DecodeError {
            index: 6, // Index of X
            byte: b'X',
        })
    );
}

#[test]
fn test_decode_chunk_8_invalid_middle_case() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0;
    decode_table[b'B' as usize] = 1;
    decode_table[b'C' as usize] = 2;

    let mut output = [0; 6];
    assert_eq!(
        decode_chunk_8(b"ABCDEFXY", 0, &decode_table, &mut output),
        Err(DecodeError {
            index: 6, // Index of X
            byte: b'X',
        })
    );
}

#[test]
fn test_decode_chunk_8_invalid_last_case() {
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    decode_table[b'A' as usize] = 0;
    decode_table[b'B' as usize] = 1;
    decode_table[b'C' as usize] = 2;
    decode_table[b'D' as usize] = 3;
    decode_table[b'E' as usize] = 4;
    decode_table[b'F' as usize] = 5;
    decode_table[b'G' as usize] = 6;

    let mut output = [0; 6];
    assert_eq!(
        decode_chunk_8(b"ABCDEFGH", 0, &decode_table, &mut output),
        Err(DecodeError {
            index: 7, // Index of H
            byte: b'H',
        })
    );
}

