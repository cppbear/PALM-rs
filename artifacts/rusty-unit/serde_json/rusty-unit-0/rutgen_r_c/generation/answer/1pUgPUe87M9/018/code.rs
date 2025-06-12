// Answer 0

#[test]
fn test_parse_line_col_invalid_column() {
    let mut msg = String::from("Error occurred at line 10 column ");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_invalid_line() {
    let mut msg = String::from("Error occurred at line not_a_number column 20");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_no_suffix() {
    let mut msg = String::from("Error occurred without location");
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
    let mut msg = String::from("Error occurred at line 15");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_only_column() {
    let mut msg = String::from("Error occurred at line 15 column");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

#[test]
fn test_parse_line_col_no_numbers() {
    let mut msg = String::from("Error occurred at line A column B");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
}

