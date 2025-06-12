// Answer 0

#[test]
fn test_parse_line_col_success() {
    let mut msg = String::from("An error occurred at line 42 column 7");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((42, 7)));
    assert_eq!(msg, "An error occurred");
}

#[test]
fn test_parse_line_col_no_at_line() {
    let mut msg = String::from("An error occurred without line information");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, msg); // msg should remain unchanged
}

#[test]
fn test_parse_line_col_no_column() {
    let mut msg = String::from("An error occurred at line 42");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, msg); // msg should remain unchanged
}

#[test]
fn test_parse_line_col_invalid_line() {
    let mut msg = String::from("An error occurred at line 42.5 column 7");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, msg); // msg should remain unchanged
}

#[test]
fn test_parse_line_col_invalid_column() {
    let mut msg = String::from("An error occurred at line 42 column 7.5");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, msg); // msg should remain unchanged
}

#[test]
fn test_parse_line_col_empty_message() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, msg); // msg should remain unchanged
}

#[test]
fn test_parse_line_col_malformed_input() {
    let mut msg = String::from("An error occurred at line ABC column DE");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, msg); // msg should remain unchanged
}

