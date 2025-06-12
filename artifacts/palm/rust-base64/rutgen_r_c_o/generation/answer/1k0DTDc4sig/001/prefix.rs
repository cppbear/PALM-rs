// Answer 0

#[test]
fn test_valid_alphabet() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[test]
fn test_alphabet_length_exact() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[should_panic]
fn test_alphabet_length_too_short() {
    let alphabet = "ABC"; // Length is less than ALPHABET_SIZE
    let _ = Alphabet::from_str_unchecked(alphabet);
}

#[should_panic]
fn test_alphabet_length_too_long() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/extracharacter"; // Length is greater than ALPHABET_SIZE
    let _ = Alphabet::from_str_unchecked(alphabet);
}

