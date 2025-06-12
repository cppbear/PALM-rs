// Answer 0

#[test]
fn test_as_str_valid_range_1() {
    let text = "Hello, World!";
    let mat = Match::new(text, 0, 5);
    mat.as_str();
}

#[test]
fn test_as_str_valid_range_2() {
    let text = "Rust Programming";
    let mat = Match::new(text, 0, 4);
    mat.as_str();
}

#[test]
fn test_as_str_valid_range_3() {
    let text = "Regex in Rust";
    let mat = Match::new(text, 6, 10);
    mat.as_str();
}

#[test]
fn test_as_str_valid_range_full_text() {
    let text = "Complete Match";
    let mat = Match::new(text, 0, text.len());
    mat.as_str();
}

#[test]
fn test_as_str_valid_range_end_index() {
    let text = "Boundary Test";
    let mat = Match::new(text, 7, text.len());
    mat.as_str();
}

#[should_panic]
fn test_as_str_panic_start_out_of_bounds() {
    let text = "Out Of Bounds";
    let mat = Match::new(text, 15, 16);
    mat.as_str();
}

#[should_panic]
fn test_as_str_panic_end_less_than_start() {
    let text = "Invalid Range";
    let mat = Match::new(text, 5, 3);
    mat.as_str();
}

#[should_panic]
fn test_as_str_panic_end_out_of_bounds() {
    let text = "Another Test";
    let mat = Match::new(text, 5, 20);
    mat.as_str();
}

