// Answer 0

#[test]
fn test_starts_with_digit_empty_string() {
    let result = starts_with_digit("");
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_whitespace_string() {
    let result = starts_with_digit("   ");
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_newline_string() {
    let result = starts_with_digit("\n");
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_tab_string() {
    let result = starts_with_digit("\t");
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_null_character_string() {
    let result = starts_with_digit("\0");
    assert_eq!(result, false);
}

