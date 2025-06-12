// Answer 0

#[test]
fn test_decode_table_empty_alphabet() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Alphabet {
        pub(crate) symbols: [u8; 64],
    }

    let empty_symbols = [0u8; 64]; // All symbols initialized but not valid
    let alphabet_empty = Alphabet { symbols: empty_symbols };
    
    let result = decode_table(&alphabet_empty);
    
    // Check that all values are INVALID_VALUE since no valid characters were mapped
    for value in result.iter() {
        assert_eq!(*value, INVALID_VALUE);
    }
}

#[test]
fn test_decode_table_valid_alphabet() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Alphabet {
        pub(crate) symbols: [u8; 64],
    }

    let valid_symbols = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
        b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
        b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
        b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
        b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
    ];
    let alphabet_valid = Alphabet { symbols: valid_symbols };
    
    let result = decode_table(&alphabet_valid);

    // Check that valid ASCII base64 characters map correctly
    for i in 0..64 {
        assert_eq!(result[alphabet_valid.symbols[i] as usize], i as u8);
    }

    // Check that non-base64 characters return INVALID_VALUE
    assert_eq!(result[b'!'] as usize, INVALID_VALUE);
}

#[test]
fn test_decode_table_edge_case() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Alphabet {
        pub(crate) symbols: [u8; 64],
    }

    let edge_symbols = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
        b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
        b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
        b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
        b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
    ];
    let alphabet_edge = Alphabet { symbols: edge_symbols };
    
    let result = decode_table(&alphabet_edge);

    // Check that the last index (63) is correctly handled
    assert_eq!(result[alphabet_edge.symbols[63] as usize], 63 as u8);
}

