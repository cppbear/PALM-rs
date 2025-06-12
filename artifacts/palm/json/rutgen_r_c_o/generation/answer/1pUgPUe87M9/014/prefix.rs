// Answer 0

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error occurred while parsing at line 12 column 34");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_invalid_suffix() {
    let mut msg = String::from("Error occurred while parsing at line 12");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_no_column_info() {
    let mut msg = String::from("Error occurred while parsing at line 12 column ");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_empty_string() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_no_line_info() {
    let mut msg = String::from("Error occurred while parsing column 34");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_column_zero() {
    let mut msg = String::from("Error occurred while parsing at line 12 column 0");
    let result = parse_line_col(&mut msg);
}

