// Answer 0

#[test]
fn test_valid_alphabet_creation() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // valid base64
    let result = Alphabet::new(alphabet);
    assert!(result.is_ok());
}

#[test]
fn test_alphabet_creation_with_min_ascii() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+!"; // valid base64 with lowest valid ASCII
    let result = Alphabet::new(alphabet);
    assert!(result.is_ok());
}

#[test]
fn test_alphabet_creation_with_max_ascii() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // valid base64 with highest valid ASCII
    let result = Alphabet::new(alphabet);
    assert!(result.is_ok());
}

#[test]
fn test_alphabet_creation_with_min_length() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::new(alphabet);
    assert_eq!(result, Ok(Alphabet::from_str_unchecked(alphabet)));
}

#[test]
fn test_alphabet_creation_with_unique_chars() {
    let alphabet = "!\"#$%&'()*+,-.0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]_^";
    let result = Alphabet::new(alphabet);
    assert!(result.is_ok());
}

