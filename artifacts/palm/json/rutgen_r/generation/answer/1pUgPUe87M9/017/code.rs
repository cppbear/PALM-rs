// Answer 0

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error occurred at line 42 column 7");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((42, 7)));
    assert_eq!(msg, "Error occurred");
}

#[test]
fn test_parse_line_col_no_line() {
    let mut msg = String::from("Error occurred");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_no_column() {
    let mut msg = String::from("Error occurred at line 10");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_invalid_line() {
    let mut msg = String::from("Error occurred at line abc column 10");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_invalid_column() {
    let mut msg = String::from("Error occurred at line 10 column xyz");
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
fn test_parse_line_col_no_space_before_column() {
    let mut msg = String::from("Error occurred at line 10column 20");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_edge_case() {
    let mut msg = String::from("Error occurred at line 0 column 0");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((0, 0)));
    assert_eq!(msg, "Error occurred");
}

