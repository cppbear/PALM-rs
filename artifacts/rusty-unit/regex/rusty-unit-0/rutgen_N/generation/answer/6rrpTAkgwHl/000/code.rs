// Answer 0

#[test]
fn test_parse_valid_regex() {
    let result = parse(r"\d+");
    assert!(result.is_ok());
}

#[test]
fn test_parse_invalid_regex() {
    let result = parse(r"\d+(");
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_string() {
    let result = parse("");
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_utf8() {
    let invalid_utf8 = "�"; // Assuming this is treated as invalid UTF-8
    let result = parse(invalid_utf8);
    assert!(result.is_ok()); // As per the function design, it allows invalid UTF-8
}

