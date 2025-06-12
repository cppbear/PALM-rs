// Answer 0

#[test]
fn test_encode_table_with_standard_alphabet() {
    struct Alphabet {
        symbols: [u8; 64],
    }

    let alphabet = Alphabet {
        symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    };

    let result = encode_table(&alphabet);
    assert_eq!(result, alphabet.symbols);
}

#[test]
fn test_encode_table_with_custom_alphabet() {
    struct Alphabet {
        symbols: [u8; 64],
    }

    let alphabet = Alphabet {
        symbols: *b"@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]_^`abcdefghijklmno",
    };

    let result = encode_table(&alphabet);
    assert_eq!(result, alphabet.symbols);
}

#[test]
fn test_encode_table_with_empty_alphabet() {
    struct Alphabet {
        symbols: [u8; 64],
    }

    let alphabet = Alphabet {
        symbols: [0; 64],
    };

    let result = encode_table(&alphabet);
    assert_eq!(result, alphabet.symbols);
}

