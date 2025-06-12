// Answer 0

#[test]
fn test_parse_line_col_invalid_line_number() {
    let mut msg = String::from("Error occurred at line 4294967295 column 0");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_missing_column_keyword() {
    let mut msg = String::from("Error occurred at line 10 instead of column 20");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_invalid_column_number() {
    let mut msg = String::from("Error occurred at line 10 column 4294967295");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_no_line_info() {
    let mut msg = String::from("Unexpected error occurred here");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_empty_message() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
}

