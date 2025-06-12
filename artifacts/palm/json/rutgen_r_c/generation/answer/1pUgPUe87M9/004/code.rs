// Answer 0

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error occurred at line 10 column 20");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((10, 20)));
    assert_eq!(msg, "Error occurred");
}

#[test]
fn test_parse_line_col_no_suffix() {
    let mut msg = String::from("Error occurred");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred");
}

#[test]
fn test_parse_line_col_missing_column() {
    let mut msg = String::from("Error occurred at line 10");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 10");
}

#[test]
fn test_parse_line_col_invalid_line() {
    let mut msg = String::from("Error occurred at line ten column 20");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line ten column 20");
}

#[test]
fn test_parse_line_col_invalid_column() {
    let mut msg = String::from("Error occurred at line 10 column twenty");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 10 column twenty");
}

