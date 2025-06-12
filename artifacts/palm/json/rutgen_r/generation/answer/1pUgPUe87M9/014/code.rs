// Answer 0

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error occurred at line 12 column 45");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((12, 45)));
    assert_eq!(msg, "Error occurred");
}

#[test]
fn test_parse_line_col_no_line() {
    let mut msg = String::from("Error occurred column 45");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred column 45");
}

#[test]
fn test_parse_line_col_no_column() {
    let mut msg = String::from("Error occurred at line 12");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 12");
}

#[test]
fn test_parse_line_col_empty_string() {
    let mut msg = String::new();
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "");
}

#[test]
fn test_parse_line_col_invalid_line_number() {
    let mut msg = String::from("Error occurred at line abc column 45");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line abc column 45");
}

#[test]
fn test_parse_line_col_invalid_column_number() {
    let mut msg = String::from("Error occurred at line 12 column xyz");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 12 column xyz");
}

#[test]
fn test_parse_line_col_no_column_suffix() {
    let mut msg = String::from("Error occurred at line 12 column");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 12 column");
}

#[test]
fn test_parse_line_col_extra_whitespace() {
    let mut msg = String::from("Error occurred at line  12  column  45");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line  12  column  45");
}

