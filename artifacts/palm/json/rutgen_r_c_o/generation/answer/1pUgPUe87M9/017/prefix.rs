// Answer 0

#[test]
fn test_parse_line_col_case_1() {
    let mut msg = String::from("Error occurred at line 1 column 1");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_case_2() {
    let mut msg = String::from("Error occurred at line 10 column 20");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_case_3() {
    let mut msg = String::from("Error occurred at line 123 column 456");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_case_4() {
    let mut msg = String::from("An error has happened at line 999 column 999");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_case_5() {
    let mut msg = String::from("Failed parsing at line 42 column 0");
    parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_case_6() {
    let mut msg = String::from("Unexpected token at line 256 column 512");
    parse_line_col(&mut msg);
}

