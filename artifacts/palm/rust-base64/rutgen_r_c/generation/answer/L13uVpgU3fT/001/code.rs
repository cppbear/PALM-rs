// Answer 0

#[test]
fn test_decode_table_valid_alphabet() {
    let alphabet = Alphabet {
        symbols: [
            b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
            b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
            b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
            b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
            b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
            b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
            b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
            b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
        ],
    };
    let result = decode_table(&alphabet);
    let expected_result: [u8; 256] = {
        let mut array = [INVALID_VALUE; 256];
        for i in 0..64 {
            array[alphabet.symbols[i] as usize] = i as u8;
        }
        array
    };
    assert_eq!(result, expected_result);
}

#[test]
fn test_decode_table_empty_alphabet() {
    let alphabet = Alphabet {
        symbols: [
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ],
    };
    let result = decode_table(&alphabet);
    let expected_result = [INVALID_VALUE; 256];
    assert_eq!(result, expected_result);
}

#[should_panic]
#[test]
fn test_decode_table_panic_condition() {
    let mut symbols = [0; 64];
    symbols[0..64].copy_from_slice(&[b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', 
                                     b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', 
                                     b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', 
                                     b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f', 
                                     b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', 
                                     b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', 
                                     b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', 
                                     b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/']);
    
    let alphabet = Alphabet {
        symbols,
    };
    // force panic by accessing an index >= 64
    alphabet.symbols[64] = b'!';
    let _ = decode_table(&alphabet); // This should panic due to index out of bounds.
}

