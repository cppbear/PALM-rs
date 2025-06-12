// Answer 0

#[test]
fn test_parse_line_col_no_line_suffix() {
    let mut msg = String::from("Error occurred");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_no_column_suffix() {
    let mut msg = String::from("Error occurred at line 10");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_invalid_suffix() {
    let mut msg = String::from("Error occurred at line 10 column");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_invalid_line_number() {
    let mut msg = String::from("Error occurred at line a column 20");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_invalid_column_number() {
    let mut msg = String::from("Error occurred at line 10 column b");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_with_suffix() {
    let mut msg = String::from("Error occurred at line 10 column 20 extra info");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((10, 20)));
    assert_eq!(msg, "Error occurred");
}

