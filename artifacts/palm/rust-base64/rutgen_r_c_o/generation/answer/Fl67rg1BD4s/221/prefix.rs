// Answer 0

#[test]
fn test_valid_decode_suffix() {
    let input: &[u8] = &[0x61, 0x62, 0x63, 0x3D]; // "abc="
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x61] = 0; // 'a'
        table[0x62] = 1; // 'b'
        table[0x63] = 2; // 'c'
        table[0x3D] = PAD_BYTE; // '='
        table
    };
    let decode_allow_trailing_bits: bool = true;
    let padding_mode = DecodePaddingMode::RequireNone;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_no_padding_and_valid_input() {
    let input: &[u8] = &[0x2f, 0x4e, 0x1c, 0x3b]; // "/N."
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x2F] = 63; // '/' -> 63
        table[0x4E] = 50; // 'N' -> 50
        table[0x1C] = INVALID_VALUE; // Invalid byte
        table[0x3B] = INVALID_VALUE; // Invalid byte
        table
    };
    let decode_allow_trailing_bits: bool = true;
    let padding_mode = DecodePaddingMode::RequireNone;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
#[should_panic]
fn test_trailing_bits_invalid_length() {
    let input: &[u8] = &[0x61, 0x62, 0x63, 0x2F]; // "abc/"
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x61] = 0; // 'a'
        table[0x62] = 1; // 'b'
        table[0x63] = 2; // 'c'
        table[0x2F] = 63; // '/' -> 63
        table
    };
    let decode_allow_trailing_bits: bool = false;
    let padding_mode = DecodePaddingMode::RequireNone;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
#[should_panic]
fn test_invalid_byte_in_input() {
    let input: &[u8] = &[0x61, 0x62, 0x63, 0x99]; // "abc" with invalid byte
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[0x61] = 0; // 'a'
        table[0x62] = 1; // 'b'
        table[0x63] = 2; // 'c'
        table
    };
    let decode_allow_trailing_bits: bool = true;
    let padding_mode = DecodePaddingMode::RequireNone;

    let _ = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

