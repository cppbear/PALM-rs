// Answer 0

#[test]
fn test_parse_line_col_no_suffix() {
    let mut msg = String::from("Error occurred");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_missing_line() {
    let mut msg = String::from("Error occurred at column 5");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_end_of_line() {
    let mut msg = String::from("Error occurred at line 10 column ");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_incomplete_column() {
    let mut msg = String::from("Error occurred at line 10 column");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_non_numeric_line() {
    let mut msg = String::from("Error occurred at line abc column 5");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_non_numeric_column() {
    let mut msg = String::from("Error occurred at line 10 column xyz");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_no_column_suffix() {
    let mut msg = String::from("Error occurred at line 10 column ");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error occurred at line 42 column 10");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((42, 10)));
}

