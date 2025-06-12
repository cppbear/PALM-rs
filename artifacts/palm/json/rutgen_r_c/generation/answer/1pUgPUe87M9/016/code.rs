// Answer 0

#[test]
fn test_parse_line_col_valid() {
    let mut msg = String::from("Error occurred while processing at line 42 column 7");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((42, 7)));
    assert_eq!(msg, "Error occurred while processing");
}

#[test]
fn test_parse_line_col_no_at_line() {
    let mut msg = String::from("Error occurred while processing");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred while processing");
}

#[test]
fn test_parse_line_col_invalid_line() {
    let mut msg = String::from("Error occurred at line xx column 5");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred");
}

#[test]
fn test_parse_line_col_no_column() {
    let mut msg = String::from("Error occurred at line 42");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 42");
}

#[test]
fn test_parse_line_col_invalid_column() {
    let mut msg = String::from("Error occurred at line 42 column yy");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 42");
}

#[test]
fn test_parse_line_col_column_number_too_large() {
    let mut msg = String::from("Error occurred at line 42 column 9999999999");
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

