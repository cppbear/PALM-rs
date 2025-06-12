// Answer 0

#[test]
fn test_decode_suffix_valid_input() {
    let input: &[u8] = &[0x41, 0x42, 0x43, 0x44]; // Corresponds to "ABCD"
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..26 {
            table[b'A' as usize + i] = i; // A-Z
            table[b'a' as usize + i] = i + 26; // a-z
        }
        for i in 0..10 {
            table[b'0' as usize + i] = i + 52; // 0-9
        }
        table[b'+' as usize] = 62; // +
        table[b'/' as usize] = 63; // /
        table
    };
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::Indifferent;
    
    decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_suffix_with_edge_case() {
    let input: &[u8] = &[0x41, 0x42, 0x3F, 0x3F]; // Corresponds to "AB??", with `3F` representing a valid last morsel
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..26 {
            table[b'A' as usize + i] = i; // A-Z
            table[b'a' as usize + i] = i + 26; // a-z
        }
        for i in 0..10 {
            table[b'0' as usize + i] = i + 52; // 0-9
        }
        table[b'+' as usize] = 62; // +
        table[b'/' as usize] = 63; // /
        table
    };
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::Indifferent;
    
    decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

#[test]
fn test_decode_suffix_with_extra_valid_characters() {
    let input: &[u8] = &[0x41, 0x42, 0x43, 0x44]; // Valid "ABCD"
    let input_index: usize = 0;
    let mut output: [u8; 4] = [0; 4];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        for i in 0..26 {
            table[b'A' as usize + i] = i; // A-Z
            table[b'a' as usize + i] = i + 26; // a-z
        }
        for i in 0..10 {
            table[b'0' as usize + i] = i + 52; // 0-9
        }
        table[b'+'] = 62; // +
        table[b'/'] = 63; // /
        table
    };
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::Indifferent;
    
    decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);
}

