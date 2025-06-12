// Answer 0

#[test]
fn test_decode_suffix_invalid_padding() {
    let input: &[u8] = &[b'A', b'B', b'=', b'='];
    let input_index: usize = 0;
    let mut output: [u8; 10] = [0; 10];
    let output_index: usize = 0;
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table
    };
    let decode_allow_trailing_bits: bool = false;
    let padding_mode: DecodePaddingMode = DecodePaddingMode::RequireNone;

    let _ = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    );
}

