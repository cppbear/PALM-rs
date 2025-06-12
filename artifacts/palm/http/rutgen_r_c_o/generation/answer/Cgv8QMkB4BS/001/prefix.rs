// Answer 0

#[test]
fn test_valid_scheme_short() {
    let input = "http";
    let _ = Scheme::from_str(input);
}

#[test]
fn test_valid_scheme_long() {
    let input = "http+example";
    let _ = Scheme::from_str(input);
}

#[test]
fn test_valid_scheme_max_length() {
    let input = "http+example:valid_scheme_with_max_length_value"; // 64 characters
    let _ = Scheme::from_str(input);
}

#[test]
fn test_invalid_scheme_too_long() {
    let input = "http+example:invalid_scheme_with_more_than_six_four_characters"; // 65 characters
    let _ = Scheme::from_str(input);
}

#[test]
fn test_empty_scheme() {
    let input = "";
    let _ = Scheme::from_str(input);
}

#[test]
fn test_invalid_scheme_space() {
    let input = "http example";
    let _ = Scheme::from_str(input);
}

#[test]
fn test_invalid_scheme_special_characters() {
    let input = "http@#!";
    let _ = Scheme::from_str(input);
}

#[test]
fn test_single_valid_character() {
    let input = "h";
    let _ = Scheme::from_str(input);
}

#[test]
fn test_uppercase_valid_character() {
    let input = "H";
    let _ = Scheme::from_str(input);
}

#[test]
fn test_invalid_character() {
    let input = "http:ä"; // includes invalid character
    let _ = Scheme::from_str(input);
}

