// Answer 0

#[derive(Debug)]
struct Alphabet {
    symbols: [u8; 64],
}

const ALPHABET_SIZE: usize = 64;

impl Alphabet {
    const fn from_str_unchecked(alphabet: &str) -> Self {
        let mut symbols = [0_u8; ALPHABET_SIZE];
        let source_bytes = alphabet.as_bytes();

        let mut index = 0;
        while index < ALPHABET_SIZE {
            symbols[index] = source_bytes[index];
            index += 1;
        }

        Self { symbols }
    }
}

#[test]
fn test_from_str_unchecked_valid() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::from_str_unchecked(alphabet);
    assert_eq!(result.symbols, alphabet.as_bytes());
}

#[test]
#[should_panic]
fn test_from_str_unchecked_too_short() {
    let alphabet = "ABC"; // Not enough characters
    let result = Alphabet::from_str_unchecked(alphabet);
    assert_eq!(result.symbols, [0; ALPHABET_SIZE]); // This won't match, expect panic
}

#[test]
#[should_panic]
fn test_from_str_unchecked_too_long() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/X"; // Too many characters
    let result = Alphabet::from_str_unchecked(alphabet);
    assert_eq!(result.symbols, [0; ALPHABET_SIZE]); // This won't match, expect panic
}

