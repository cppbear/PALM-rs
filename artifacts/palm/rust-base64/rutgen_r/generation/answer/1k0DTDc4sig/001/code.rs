// Answer 0

#[test]
fn test_from_str_unchecked_valid() {
    const ALPHABET_SIZE: usize = 64; // assuming a base64-like alphabet size
    struct Alphabet {
        symbols: [u8; ALPHABET_SIZE],
    }

    const fn from_str_unchecked(alphabet: &str) -> Alphabet {
        let mut symbols = [0_u8; ALPHABET_SIZE];
        let source_bytes = alphabet.as_bytes();

        let mut index = 0;
        while index < ALPHABET_SIZE {
            symbols[index] = source_bytes[index];
            index += 1;
        }

        Alphabet { symbols }
    }

    let valid_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // A valid base64 alphabet
    let result = from_str_unchecked(valid_alphabet);
    assert_eq!(result.symbols, valid_alphabet.as_bytes());
}

#[should_panic]
#[test]
fn test_from_str_unchecked_out_of_bounds() {
    const ALPHABET_SIZE: usize = 64;
    struct Alphabet {
        symbols: [u8; ALPHABET_SIZE],
    }

    const fn from_str_unchecked(alphabet: &str) -> Alphabet {
        let mut symbols = [0_u8; ALPHABET_SIZE];
        let source_bytes = alphabet.as_bytes();

        let mut index = 0;
        while index < ALPHABET_SIZE {
            symbols[index] = source_bytes[index];
            index += 1;
        }

        Alphabet { symbols }
    }

    let invalid_alphabet = "TOO_SHORT";
    from_str_unchecked(invalid_alphabet); // this will panic as it doesn't satisfy the required size.
}

