// Answer 0

#[test]
fn test_valid_alphabet_edge_case_low() {
    let alphabet = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]^_`abcdef";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_edge_case_high() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_middle_range() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+!@#$%^&*()_";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_valid_alphabet_with_special_chars() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-:.?!'()$&*";
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_invalid_length() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"; // 65 characters
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_unprintable_character() {
    let alphabet = "ADEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789*"; // contains unprintable
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_duplicate_character() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"; // 'A' is duplicated
    let result = Alphabet::new(alphabet);
}

#[test]
fn test_reserved_character() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+=="; // contains '='
    let result = Alphabet::new(alphabet);
}

