// Answer 0

#[test]
fn test_decode_suffix_invalid_padding() {
    use crate::engine::general_purpose::{decode_suffix, DecodeMetadata, DecodePaddingMode, DecodeSliceError, DecodeError};

    const PAD_BYTE: u8 = b'='; 
    const INVALID_VALUE: u8 = 255; 
    const DECODE_TABLE: [u8; 256] = {
        let mut table = [INVALID_VALUE; 256];
        table[b'A' as usize] = 0;
        table[b'B' as usize] = 1;
        table[b'C' as usize] = 2;
        table[b'D' as usize] = 3;
        table[b'E' as usize] = 4;
        table[b'F' as usize] = 5;
        table[b'G' as usize] = 6;
        table[b'H' as usize] = 7;
        table[b'I' as usize] = 8;
        table[b'J' as usize] = 9;
        table[b'K' as usize] = 10;
        table[b'L' as usize] = 11;
        table[b'M' as usize] = 12;
        table[b'N' as usize] = 13;
        table[b'O' as usize] = 14;
        table[b'P' as usize] = 15;
        table[b'Q' as usize] = 16;
        table[b'R' as usize] = 17;
        table[b'S' as usize] = 18;
        table[b'T' as usize] = 19;
        table[b'U' as usize] = 20;
        table[b'V' as usize] = 21;
        table[b'W' as usize] = 22;
        table[b'X' as usize] = 23;
        table[b'Y' as usize] = 24;
        table[b'Z' as usize] = 25;
        table[b'a' as usize] = 26;
        table[b'b' as usize] = 27;
        table[b'c' as usize] = 28;
        table[b'd' as usize] = 29;
        table[b'e' as usize] = 30;
        table[b'f' as usize] = 31;
        table[b'g' as usize] = 32;
        table[b'h' as usize] = 33;
        table[b'i' as usize] = 34;
        table[b'j' as usize] = 35;
        table[b'k' as usize] = 36;
        table[b'l' as usize] = 37;
        table[b'm' as usize] = 38;
        table[b'n' as usize] = 39;
        table[b'o' as usize] = 40;
        table[b'p' as usize] = 41;
        table[b'q' as usize] = 42;
        table[b'r' as usize] = 43;
        table[b's' as usize] = 44;
        table[b't' as usize] = 45;
        table[b'u' as usize] = 46;
        table[b'v' as usize] = 47;
        table[b'w' as usize] = 48;
        table[b'x' as usize] = 49;
        table[b'y' as usize] = 50;
        table[b'z' as usize] = 51;
        table[b'0' as usize] = 52;
        table[b'1' as usize] = 53;
        table[b'2' as usize] = 54;
        table[b'3' as usize] = 55;
        table[b'4' as usize] = 56;
        table[b'5' as usize] = 57;
        table[b'6' as usize] = 58;
        table[b'7' as usize] = 59;
        table[b'8' as usize] = 60;
        table[b'9' as usize] = 61;
        table[b'+' as usize] = 62;
        table[b'/' as usize] = 63;
        table
    };

    let input: &[u8] = b"QUJD"; 
    let input_index = 0;
    let mut output: [u8; 4] = [0; 4];
    let mut output_index = 0;
    let decode_allow_trailing_bits = false;
    let padding_mode = DecodePaddingMode::RequireNone; 
    
    let result = decode_suffix(
        input,
        input_index,
        &mut output,
        output_index,
        &DECODE_TABLE,
        decode_allow_trailing_bits,
        padding_mode,
    );

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DecodeError::InvalidPadding.into());
}

