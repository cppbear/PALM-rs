// Answer 0

#[test]
fn test_parse_line_col_success() {
    let mut msg = String::from("Error occurred at line 42 column 7");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((42, 7)));
    assert_eq!(msg, "Error occurred");
}

#[test]
fn test_parse_line_col_no_line() {
    let mut msg = String::from("Error occurred at column 7");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at column 7");
}

#[test]
fn test_parse_line_col_no_column() {
    let mut msg = String::from("Error occurred at line 42");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 42");
}

#[test]
fn test_parse_line_col_non_numeric_line() {
    let mut msg = String::from("Error occurred at line x column 3");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line x column 3");
}

#[test]
fn test_parse_line_col_non_numeric_column() {
    let mut msg = String::from("Error occurred at line 42 column y");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 42 column y");
}

#[test]
fn test_parse_line_col_empty_message() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "");
}

#[test]
fn test_parse_line_col_invalid_suffix() {
    let mut msg = String::from("Error not at line 42 column 7");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error not at line 42 column 7");
}

