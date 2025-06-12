// Answer 0

#[test]
fn test_parse_line_col_valid() {
    let mut msg = String::from("Error occurred here at line 12 column 34");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((12, 34)));
    assert_eq!(msg, "Error occurred here ");
}

#[test]
fn test_parse_line_col_no_line() {
    let mut msg = String::from("Error occurred here column 34");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_no_column() {
    let mut msg = String::from("Error occurred here at line 12");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_invalid_line() {
    let mut msg = String::from("Error occurred here at line abc column 34");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_invalid_column() {
    let mut msg = String::from("Error occurred here at line 12 column xyz");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_empty_message() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_only_suffix() {
    let mut msg = String::from("Error occurred here at line 1 column 2");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((1, 2)));
    assert_eq!(msg, "Error occurred here ");
}

