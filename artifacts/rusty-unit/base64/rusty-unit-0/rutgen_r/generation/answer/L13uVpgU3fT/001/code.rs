// Answer 0

#[test]
fn test_decode_table_valid_alphabet() {
    struct Alphabet {
        symbols: [u8; 64],
    }

    const VALID_SYMBOLS: [u8; 64] = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
        b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
        b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
        b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
        b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
    ];

    let alphabet = Alphabet { symbols: VALID_SYMBOLS };

    let table = decode_table(&alphabet);

    for index in 0..64 {
        assert_eq!(table[alphabet.symbols[index] as usize], index as u8);
    }
    for index in 64..256 {
        assert_eq!(table[index], INVALID_VALUE);
    }
}

#[should_panic]
#[test]
fn test_decode_table_invalid_alphabet() {
    struct Alphabet {
        symbols: [u8; 65], // This exceeds 64 and should panic on bound check
    }

    const INVALID_SYMBOLS: [u8; 65] = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
        b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
        b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
        b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
        b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
        b'=', // Additional invalid index
    ];

    let alphabet = Alphabet { symbols: INVALID_SYMBOLS };
    
    decode_table(&alphabet); // This should panic due to out-of-bounds access
}

