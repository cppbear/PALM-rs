// Answer 0

#[test]
fn test_make_error_valid_input_1() {
    let msg = String::from("Error occurred at line 1 column 1");
    let error = make_error(msg);
}

#[test]
fn test_make_error_valid_input_2() {
    let msg = String::from("Error occurred at line 2 column 10");
    let error = make_error(msg);
}

#[test]
fn test_make_error_valid_input_3() {
    let msg = String::from("Error occurred at line 1000 column 2000");
    let error = make_error(msg);
}

#[test]
fn test_make_error_valid_input_4() {
    let msg = String::from("Random error message at line 0 column 0");
    let error = make_error(msg);
}

#[test]
fn test_make_error_no_positions() {
    let msg = String::from("No positions here");
    let error = make_error(msg);
}

#[test]
#[should_panic]
fn test_make_error_edge_case_empty_string() {
    let msg = String::from("");
    let error = make_error(msg);
}

#[test]
#[should_panic]
fn test_make_error_edge_case_no_line_column() {
    let msg = String::from("Some random error message with no line or column info");
    let error = make_error(msg);
}

