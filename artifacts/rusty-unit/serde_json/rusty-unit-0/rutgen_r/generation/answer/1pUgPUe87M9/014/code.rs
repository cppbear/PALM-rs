// Answer 0

#[test]
fn test_parse_line_col_no_suffix() {
    let mut msg = String::from("Error occurred.");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_no_line_indicator() {
    let mut msg = String::from("Error occurred at column 5.");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_no_column_indicator() {
    let mut msg = String::from("Error occurred at line 10.");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_invalid_line_format() {
    let mut msg = String::from("Error occurred at line xyz column 5");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_invalid_column_format() {
    let mut msg = String::from("Error occurred at line 10 column abc");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error occurred at line 12 column 34");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((12, 34)));
    assert_eq!(msg, "Error occurred ");
}

#[test]
fn test_parse_line_col_empty_message() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_only_suffix() {
    let mut msg = String::from(" at line 10 column 5");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

