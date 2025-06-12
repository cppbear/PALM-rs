// Answer 0

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error occurred at line 42 column 7");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((42, 7)));
    assert_eq!(msg, "Error occurred");
}

#[test]
fn test_parse_line_col_no_at_line() {
    let mut msg = String::from("Error occurred at position 42 column 7");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at position 42 column 7");
}

#[test]
fn test_parse_line_col_no_at_column() {
    let mut msg = String::from("Error occurred at line 42 not a column");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 42 not a column");
}

#[test]
fn test_parse_line_col_invalid_line_number() {
    let mut msg = String::from("Error occurred at line abc column 5");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line abc column 5");
}

#[test]
fn test_parse_line_col_invalid_column_number() {
    let mut msg = String::from("Error occurred at line 42 column def");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 42 column def");
}

#[test]
fn test_parse_line_col_correct_format_no_digits_after_column() {
    let mut msg = String::from("Error occurred at line 42 column ");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 42 column ");
}

#[test]
fn test_parse_line_col_missing_column() {
    let mut msg = String::from("Error occurred at line 42");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 42");
}

#[test]
fn test_parse_line_col_empty_message() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "");
}

