// Answer 0

#[test]
fn test_parse_line_col_no_at_line() {
    let mut msg = String::from("Some error occurred.");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_ends_before_column() {
    let mut msg = String::from("Error message without position info at line 10 column");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_non_numeric_value_after_at_line() {
    let mut msg = String::from("Error occurred at line ten column 5");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_digits_followed_by_non_digit_after_at_line() {
    let mut msg = String::from("Error occurred at line 12a column 5");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_starts_with_at_line() {
    let mut msg = String::from(" at line 10 column 5");
    parse_line_col(&mut msg);
}

