// Answer 0

#[test]
fn test_decode_suffix_padding_case() {
    let input: &[u8] = b"YmF=======";
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 2;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::Indifferent;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_suffix_no_padding_case() {
    let input: &[u8] = b"YmF";
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 2;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::Indifferent;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_suffix_invalid_padding_case() {
    let input: &[u8] = b"YmF=Y=";
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 2;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::Indifferent;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_suffix_correct_morsels() {
    let input: &[u8] = b"Y3Q=";
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::Indifferent;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_suffix_trailing_bits_disallowed() {
    let input: &[u8] = b"Yw==";
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::Indifferent;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_suffix_with_extra_input() {
    let input: &[u8] = b"YmF=extra";
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 2;
    let decode_table: [u8; 256] = [INVALID_VALUE; 256];
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::Indifferent;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

