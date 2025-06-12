// Answer 0

#[test]
fn test_from_str_valid_regex() {
    let pattern = r"^\d+$";
    let result = from_str(pattern);
    assert!(result.is_ok());
}

#[test]
fn test_from_str_invalid_regex() {
    let invalid_pattern = r"[\d+";
    let result = from_str(invalid_pattern);
    assert!(result.is_err());
}

#[test]
fn test_from_str_empty_string() {
    let empty_pattern = "";
    let result = from_str(empty_pattern);
    assert!(result.is_err());
}

#[test]
fn test_from_str_single_character() {
    let single_char_pattern = "a";
    let result = from_str(single_char_pattern);
    assert!(result.is_ok());
}

#[test]
fn test_from_str_special_characters() {
    let special_char_pattern = r"\d{2,4}";
    let result = from_str(special_char_pattern);
    assert!(result.is_ok());
}

