// Answer 0

#[test]
fn test_from_str_valid_regex() {
    let regex_str = r"^\d{3}-\d{2}-\d{4}$";
    let result = from_str(regex_str);
    assert!(result.is_ok());
}

#[test]
fn test_from_str_empty_string() {
    let regex_str = "";
    let result = from_str(regex_str);
    assert!(result.is_err());
}

#[test]
fn test_from_str_invalid_regex() {
    let regex_str = "[a-z";
    let result = from_str(regex_str);
    assert!(result.is_err());
}

#[test]
fn test_from_str_simple_regex() {
    let regex_str = "hello";
    let result = from_str(regex_str);
    assert!(result.is_ok());
}

#[test]
fn test_from_str_special_characters() {
    let regex_str = r"\d+\s\w+"; // Matches digits followed by any word character
    let result = from_str(regex_str);
    assert!(result.is_ok());
}

#[test]
fn test_from_str_boundary_conditions() {
    let regex_str = ".+";
    let result = from_str(regex_str);
    assert!(result.is_ok());
}

