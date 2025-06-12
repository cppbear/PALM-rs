// Answer 0

#[test]
fn test_from_str_unchecked_valid() {
    const VALID_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let alphabet = Alphabet::from_str_unchecked(VALID_ALPHABET);
    assert_eq!(alphabet.symbols, VALID_ALPHABET.as_bytes());
}

#[test]
fn test_from_str_unchecked_invalid_length() {
    const INVALID_ALPHABET: &str = "ShortAlphabet";
    let alphabet = Alphabet::from_str_unchecked(INVALID_ALPHABET);
    assert_ne!(alphabet.symbols.len(), 64);
}

#[test]
fn test_from_str_unchecked_boundary_check() {
    const BOUNDARY_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-/";
    let alphabet = Alphabet::from_str_unchecked(BOUNDARY_ALPHABET);
    assert_eq!(alphabet.symbols[ALPHABET_SIZE - 1], b'/');
}

