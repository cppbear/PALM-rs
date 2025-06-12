// Answer 0

#[test]
fn test_decode_suffix_invalid_last_symbol() {
    let input: &[u8] = b"ABCD"; // 4 bytes input
    let decode_table: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        // Assume A, B, C, D can be decoded to some values
        table[b'A' as usize] = 0; // Assume valid decoding value
        table[b'B' as usize] = 1; // Assume valid decoding value
        table[b'C' as usize] = 2; // Assume valid decoding value
        table[b'D' as usize] = 3; // Assume valid decoding value
        table
    };
    
    let mut output = [0u8; 4];
    let input_index = 0;
    let output_index = 0;
    
    // Set decode_allow_trailing_bits to false and padding_mode to Indifferent
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::Indifferent;

    // Add a last symbol that triggers the InvalidLastSymbol error.
    // Assuming D (3) last symbol has trailing bits set.
    let result = decode_suffix(input, input_index, &mut output, output_index, &decode_table, decode_allow_trailing_bits, padding_mode);

    assert_eq!(
        result,
        Err(DecodeSliceError::DecodeError(DecodeError::InvalidLastSymbol(
            input_index + 2, // morsels_in_leftover is 2, thus input_index + morsels_in_leftover - 1 = 0 + 2
            b'D' // last_symbol is b'D'
        )))
    );
}

