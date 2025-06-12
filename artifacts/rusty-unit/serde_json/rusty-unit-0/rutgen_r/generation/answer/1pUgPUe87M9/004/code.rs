// Answer 0

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error occurred at line 42 column 7");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((42, 7)));
}

#[test]
fn test_parse_line_col_no_line() {
    let mut msg = String::from("Error occurred column 7");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_no_column() {
    let mut msg = String::from("Error occurred at line 42");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_invalid_line() {
    let mut msg = String::from("Error occurred at line abc column 7");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_invalid_column() {
    let mut msg = String::from("Error occurred at line 42 column xyz");
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
fn test_parse_line_col_only_line() {
    let mut msg = String::from("Error occurred at line 123");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_no_suffix() {
    let mut msg = String::from("Error occurred");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

