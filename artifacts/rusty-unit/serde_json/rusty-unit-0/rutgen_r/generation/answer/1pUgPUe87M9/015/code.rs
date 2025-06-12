// Answer 0

#[test]
fn test_parse_line_col_with_invalid_line_number() {
    let mut msg = String::from("Error occurred at line abc column 5");
    assert_eq!(parse_line_col(&mut msg), None);
}

#[test]
fn test_parse_line_col_with_no_column() {
    let mut msg = String::from("Error occurred at line 1");
    assert_eq!(parse_line_col(&mut msg), None);
}

#[test]
fn test_parse_line_col_with_missing_at_line() {
    let mut msg = String::from("Error occurred column 2");
    assert_eq!(parse_line_col(&mut msg), None);
}

#[test]
fn test_parse_line_col_with_panic_on_end_slice() {
    let mut msg = String::from("Error occurred at line 1 column ");
    assert_eq!(parse_line_col(&mut msg), None);
}

#[test]
fn test_parse_line_col_with_empty_message() {
    let mut msg = String::new();
    assert_eq!(parse_line_col(&mut msg), None);
}

#[test]
fn test_parse_line_col_with_correct_input() {
    let mut msg = String::from("Error occurred at line 12 column 34");
    assert_eq!(parse_line_col(&mut msg), Some((12, 34)));
}

