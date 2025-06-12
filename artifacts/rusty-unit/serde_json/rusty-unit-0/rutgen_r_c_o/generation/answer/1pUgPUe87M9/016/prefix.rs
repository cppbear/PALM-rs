// Answer 0

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error occurred at line 5 column 10");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_no_line() {
    let mut msg = String::from("Error occurred column 10");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_no_column() {
    let mut msg = String::from("Error occurred at line 5");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_invalid_line() {
    let mut msg = String::from("Error occurred at line x column 10");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_invalid_column() {
    let mut msg = String::from("Error occurred at line 5 column y");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_non_numeric_line() {
    let mut msg = String::from("Error occurred at line -1 column 10");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_non_numeric_column() {
    let mut msg = String::from("Error occurred at line 5 column 10a");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_empty_msg() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_missing_at_line() {
    let mut msg = String::from("Error occurred at col 10");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_unmatched_column() {
    let mut msg = String::from("Error occurred at line 5 column 10 extra");
    let result = parse_line_col(&mut msg);
}

