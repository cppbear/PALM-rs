// Answer 0

#[test]
fn test_parse_line_col_no_line_found() {
    let mut msg = String::from("Error occurred without a line indication");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_no_column_prefix() {
    let mut msg = String::from("An error occurred at line 10, but no column information");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_invalid_format() {
    let mut msg = String::from("An error happened at line 5 column");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_empty_msg() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

