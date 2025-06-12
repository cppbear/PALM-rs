// Answer 0

#[test]
fn test_parse_line_col_basic() {
    let mut msg = String::from("error occurred at line 0 column 0");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_single_digit() {
    let mut msg = String::from("warning: something went wrong at line 1 column 1");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_multiple_digits() {
    let mut msg = String::from("error occurred at line 9 column 99");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_large_numbers() {
    let mut msg = String::from("fatal: unexpected error at line 429 column 256");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_max_value() {
    let mut msg = String::from("critical failure at line 4294967295 column 4294967295");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_no_suffix() {
    let mut msg = String::from("just a regular message");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_invalid_format() {
    let mut msg = String::from("error occurred at line two column five");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_no_column() {
    let mut msg = String::from("error occurred at line 10");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_empty_string() {
    let mut msg = String::from("");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_no_line_info() {
    let mut msg = String::from("error column 10");
    parse_line_col(&mut msg);
}

