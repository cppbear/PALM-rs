// Answer 0

#[test]
fn test_parse_line_col_no_line_info() {
    let mut msg = String::from("Error occurred due to invalid format.");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_empty_string() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_no_column_info() {
    let mut msg = String::from("Error at line 10 something went wrong.");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_only_line_info() {
    let mut msg = String::from("Error occurred at line 5");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

