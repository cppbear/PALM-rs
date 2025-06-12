// Answer 0

#[test]
fn test_parse_line_col_valid() {
    let mut message = String::from("Error occurred at line 42 column 3");
    let result = parse_line_col(&mut message);
    assert_eq!(result, Some((42, 3)));
    assert_eq!(message, "Error occurred");
}

#[test]
fn test_parse_line_col_missing_line() {
    let mut message = String::from("Error occurred column 3");
    let result = parse_line_col(&mut message);
    assert_eq!(result, None);
    assert_eq!(message, "Error occurred column 3");
}

#[test]
fn test_parse_line_col_missing_column() {
    let mut message = String::from("Error occurred at line 42");
    let result = parse_line_col(&mut message);
    assert_eq!(result, None);
    assert_eq!(message, "Error occurred at line 42");
}

#[test]
fn test_parse_line_col_non_digit_line() {
    let mut message = String::from("Error occurred at line abc column 3");
    let result = parse_line_col(&mut message);
    assert_eq!(result, None);
    assert_eq!(message, "Error occurred at line abc column 3");
}

#[test]
fn test_parse_line_col_non_digit_column() {
    let mut message = String::from("Error occurred at line 42 column xyz");
    let result = parse_line_col(&mut message);
    assert_eq!(result, None);
    assert_eq!(message, "Error occurred at line 42 column xyz");
}

#[test]
fn test_parse_line_col_empty_string() {
    let mut message = String::from("");
    let result = parse_line_col(&mut message);
    assert_eq!(result, None);
    assert_eq!(message, "");
}

#[test]
fn test_parse_line_col_invalid_format() {
    let mut message = String::from("Error at line column 3");
    let result = parse_line_col(&mut message);
    assert_eq!(result, None);
    assert_eq!(message, "Error at line column 3");
}

#[test]
fn test_parse_line_col_excessive_digits() {
    let mut message = String::from("Error occurred at line 4294967296 column 3");
    let result = parse_line_col(&mut message); // Assuming max usize can process 4294967296 successfully.
    assert_eq!(result, Some((4294967296 - 1, 3)));  // Test the boundary
    assert_eq!(message, "Error occurred"); // Confirm truncation
}

