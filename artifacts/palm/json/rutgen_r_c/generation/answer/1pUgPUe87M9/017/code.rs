// Answer 0

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error occurred at line 10 column 20");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((10, 20)));
    assert_eq!(msg, "Error occurred");
}

#[test]
fn test_parse_line_col_no_line_info() {
    let mut msg = String::from("No line information");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "No line information");
}

#[test]
fn test_parse_line_col_no_column_info() {
    let mut msg = String::from("Error occurred at line 10");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 10");
}

#[test]
fn test_parse_line_col_incomplete_column_info() {
    let mut msg = String::from("Error occurred at line 10 column");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 10 column");
}

#[test]
fn test_parse_line_col_invalid_line_number() {
    let mut msg = String::from("Error occurred at line ab column 20");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line ab column 20");
}

#[test]
fn test_parse_line_col_invalid_column_number() {
    let mut msg = String::from("Error occurred at line 10 column cd");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 10 column cd");
}

