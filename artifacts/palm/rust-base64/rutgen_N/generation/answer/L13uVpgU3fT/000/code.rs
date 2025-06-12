// Answer 0

#[derive(Debug)]
struct Alphabet {
    symbols: [u8; 64],
}

const INVALID_VALUE: u8 = 255;

#[test]
fn test_decode_table_valid_alphabet() {
    let alphabet = Alphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };
    let table = decode_table(&alphabet);

    for i in 0..64 {
        assert_eq!(table[alphabet.symbols[i] as usize], i as u8);
    }
}

#[test]
fn test_decode_table_invalid_bytes() {
    let alphabet = Alphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };
    let table = decode_table(&alphabet);

    for i in 0..256 {
        if (i as usize) < alphabet.symbols.len() && alphabet.symbols.contains(&(i as u8)) {
            // Valid base64 characters
            continue;
        }
        assert_eq!(table[i], INVALID_VALUE);
    }
}

#[test]
fn test_decode_table_empty_alphabet() {
    let alphabet = Alphabet {
        symbols: [0; 64],
    };
    let table = decode_table(&alphabet);

    for i in 0..256 {
        assert_eq!(table[i], INVALID_VALUE);
    }
}

