// Answer 0

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error occurred at line 10 column 5");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, Some((10, 5)));
    assert_eq!(msg, "Error occurred ");
}

#[test]
fn test_parse_line_col_no_suffix() {
    let mut msg = String::from("Error occurred without location");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred without location");
}

#[test]
fn test_parse_line_col_invalid_line_number() {
    let mut msg = String::from("Error occurred at line not_a_number column 5");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line not_a_number column 5");
}

#[test]
fn test_parse_line_col_invalid_column_number() {
    let mut msg = String::from("Error occurred at line 10 column not_a_number");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 10 column not_a_number");
}

#[test]
fn test_parse_line_col_no_column_suffix() {
    let mut msg = String::from("Error occurred at line 10");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "Error occurred at line 10");
}

#[test]
fn test_parse_line_col_edge_case_empty_string() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "");
}

#[test]
fn test_parse_line_col_edge_case_wrong_format() {
    let mut msg = String::from("line 10 column 5 at Error occurred.");
    let result = parse_line_col(&mut msg);
    assert_eq!(result, None);
    assert_eq!(msg, "line 10 column 5 at Error occurred.");
}

