// Answer 0

#[test]
fn test_escape_empty_string() {
    let input = "";
    escape(input);
}

#[test]
fn test_escape_single_character() {
    let input = "a";
    escape(input);
}

#[test]
fn test_escape_special_characters_only() {
    let input = ".*+?|(){}[]^$\\";
    escape(input);
}

#[test]
fn test_escape_alphanumeric_mix() {
    let input = "abc123!@#";
    escape(input);
}

#[test]
fn test_escape_unicode_characters() {
    let input = "こんにちは";
    escape(input);
}

#[test]
fn test_escape_very_long_string() {
    let input = "a".repeat(1_000_000);
    escape(&input);
}

#[test]
fn test_escape_large_number_of_special_characters() {
    let input = "!@#$%^&*()_+<>?:\"{}[];','|\\\\".repeat(1000);
    escape(&input);
}

