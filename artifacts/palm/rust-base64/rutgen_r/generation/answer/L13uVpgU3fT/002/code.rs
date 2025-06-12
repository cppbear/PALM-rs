// Answer 0

#[test]
fn test_decode_table_with_full_alphabet() {
    struct Alphabet {
        symbols: [u8; 64],
    }

    const INVALID_VALUE: u8 = 255; // Assuming INVALID_VALUE is defined as 255

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
    for index in 0..64 {
        assert_eq!(result[alphabet.symbols[index] as usize], index as u8);
    }

    for symbol in 0..=255 {
        if !alphabet.symbols.contains(&(symbol as u8)) {
            assert_eq!(result[symbol], INVALID_VALUE);
        }
    }
}

#[test]
#[should_panic]
fn test_decode_table_with_invalid_alphabet_length() {
    struct Alphabet {
        symbols: [u8; 65], // Invalid length
    }

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
            b'!', // Extra symbol causes overflow
        ],
    };

    let _ = decode_table(&alphabet);
}

