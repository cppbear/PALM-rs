// Answer 0

#[test]
fn test_decode_suffix_valid_case_with_full_padding() {
    let input: &[u8] = &[b'A', b'B', b'=', b'=', b'C'];
    let input_index: usize = 0;
    let mut output: [u8; 6] = [0; 6];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        // fill other necessary values
        table
    };
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::RequireCanonical;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_suffix_valid_case_with_no_padding() {
    let input: &[u8] = &[b'A', b'B', b'C', b'D'];
    let input_index: usize = 0;
    let mut output: [u8; 6] = [0; 6];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        // fill other necessary values
        table
    };
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::RequireNone;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
#[should_panic]
fn test_decode_suffix_invalid_padding_too_early() {
    let input: &[u8] = &[b'A', b'B', b'=', b'C'];
    let input_index: usize = 0;
    let mut output: [u8; 6] = [0; 6];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        // fill other necessary values
        table
    };
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::RequireCanonical;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
#[should_panic]
fn test_decode_suffix_invalid_last_symbol() {
    let input: &[u8] = &[b'A', b'B', b'F'];
    let input_index: usize = 0;
    let mut output: [u8; 6] = [0; 6];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        // add invalid characters handling for the edge case
        table[b'F' as usize] = INVALID_VALUE;
        table
    };
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::Indifferent;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_suffix_edge_case_one_valid_padding() {
    let input: &[u8] = &[b'A', b'B', b'C', b'=' ];
    let input_index: usize = 0;
    let mut output: [u8; 6] = [0; 6];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table
    };
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::RequireCanonical;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

