// Answer 0

#[test]
fn test_parse_line_col_no_line_col() {
    let mut msg = String::from("Error occurred");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_empty_string() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_no_suffix() {
    let mut msg = String::from("An error happened at some point.");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_no_line_keyword() {
    let mut msg = String::from("Error at column 10");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_no_column_keyword() {
    let mut msg = String::from("Error occurred at line 5");
    let result = parse_line_col(&mut msg);
}

