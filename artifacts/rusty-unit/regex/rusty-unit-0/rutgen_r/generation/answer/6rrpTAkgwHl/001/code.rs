// Answer 0

#[test]
fn test_parse_valid_utf8() {
    let re = "^[a-zA-Z0-9]+$";
    let result = parse(re);
    assert!(result.is_ok());
}

#[test]
fn test_parse_invalid_utf8() {
    let re = "\u{FFFF}"; // Invalid UTF-8 character
    let result = parse(re);
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_string() {
    let re = "";
    let result = parse(re);
    assert!(result.is_ok());
}

#[test]
fn test_parse_whitespace_only() {
    let re = "   ";
    let result = parse(re);
    assert!(result.is_ok());
}

#[test]
fn test_parse_special_characters() {
    let re = ".*?[^a-zA-Z0-9]";
    let result = parse(re);
    assert!(result.is_ok());
}

#[test]
fn test_parse_very_long_string() {
    let re = "a".repeat(1000); // Long valid regex
    let result = parse(&re);
    assert!(result.is_ok());
}

#[test]
fn test_parse_nested_quantifiers() {
    let re = "(a+)*";
    let result = parse(re);
    assert!(result.is_ok());
}

#[test]
fn test_parse_backreference() {
    let re = r"(\d)\1";
    let result = parse(re);
    assert!(result.is_ok());
}

#[test]
fn test_parse_unbalanced_parentheses() {
    let re = "(abc";
    let result = parse(re);
    assert!(result.is_err());
} 

#[test]
fn test_parse_invalid_character_class() {
    let re = "[a-z-]";
    let result = parse(re);
    assert!(result.is_ok());
} 

#[test]
#[should_panic]
fn test_parse_high_unicode() {
    let re = "\u{10FFFF}"; // Outside the valid UTF-8 range
    let result = parse(re);
    assert!(result.is_err());
}

