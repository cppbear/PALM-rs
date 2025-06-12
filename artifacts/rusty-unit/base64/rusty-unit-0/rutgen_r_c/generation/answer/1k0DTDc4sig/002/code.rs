// Answer 0

#[test]
fn test_from_str_unchecked_valid_alphabet() {
    const VALID_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let alphabet = Alphabet::from_str_unchecked(VALID_ALPHABET);
    assert_eq!(alphabet.symbols, VALID_ALPHABET.as_bytes());
}

#[test]
#[should_panic]
fn test_from_str_unchecked_boundary_exceeding_length() {
    const INVALID_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // 65 characters
    let _ = Alphabet::from_str_unchecked(INVALID_ALPHABET);
}

#[test]
fn test_from_str_unchecked_alphabet_with_exact_length() {
    const ALPHABET_EXACT_LENGTH: &str = "ABCDEFGHIJKLMNPQRSTUVWXYZabcdefghijklmno";
    let alphabet = Alphabet::from_str_unchecked(ALPHABET_EXACT_LENGTH);
    assert_eq!(alphabet.symbols, ALPHABET_EXACT_LENGTH.as_bytes());
}

