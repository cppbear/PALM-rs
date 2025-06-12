// Answer 0

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error occurred at line 12 column 34");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((12, 34)));
    assert_eq!(msg, "Error occurred");
}

#[test]
fn test_parse_line_col_missing_line() {
    let mut msg = String::from("Error occurred column 34");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred column 34");
}

#[test]
fn test_parse_line_col_missing_column() {
    let mut msg = String::from("Error occurred at line 12");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 12");
}

#[test]
fn test_parse_line_col_invalid_line_number() {
    let mut msg = String::from("Error occurred at line abc column 34");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line abc column 34");
}

#[test]
fn test_parse_line_col_invalid_column_number() {
    let mut msg = String::from("Error occurred at line 12 column xyz");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 12 column xyz");
}

#[test]
fn test_parse_line_col_no_suffix() {
    let mut msg = String::from("Error occurred at line 12 column");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 12 column");
}

#[test]
fn test_parse_line_col_empty_string() {
    let mut msg = String::new();
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "");
}

#[test]
fn test_parse_line_col_space_in_column() {
    let mut msg = String::from("Error occurred at line 12 column 34 more text");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((12, 34)));
    assert_eq!(msg, "Error occurred");
}

#[test]
fn test_parse_line_col_boundary_conditions() {
    let mut msg = String::from("Error occurred at line 0 column 0");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((0, 0)));
    assert_eq!(msg, "Error occurred");

    msg = String::from("Error occurred at line 99999 column 99999");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((99999, 99999)));
    assert_eq!(msg, "Error occurred");
}

