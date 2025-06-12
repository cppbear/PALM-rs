// Answer 0

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error message at line 34 column 10");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((34, 10)));
    assert_eq!(msg, "Error message ");
}

#[test]
fn test_parse_line_col_no_suffix() {
    let mut msg = String::from("Error message without line and column");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, msg);
}

#[test]
fn test_parse_line_col_missing_line_info() {
    let mut msg = String::from("Error message at column 10");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, msg);
}

#[test]
fn test_parse_line_col_invalid_line() {
    let mut msg = String::from("Error message at line XXX column 10");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, msg);
}

#[test]
fn test_parse_line_col_invalid_column() {
    let mut msg = String::from("Error message at line 34 column YYY");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, msg);
}

#[test]
fn test_parse_line_col_extra_data_after_columns() {
    let mut msg = String::from("Error message at line 34 column 10 with extra data");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((34, 10)));
    assert_eq!(msg, "Error message ");
}

