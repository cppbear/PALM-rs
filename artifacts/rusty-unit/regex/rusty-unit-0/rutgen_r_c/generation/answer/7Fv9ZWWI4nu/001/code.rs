// Answer 0

#[test]
fn test_as_str_valid_range() {
    let text = "hello world";
    let m = Match::new(text, 0, 5);
    assert_eq!(m.as_str(), "hello");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_as_str_start_out_of_bounds() {
    let text = "hello world";
    let m = Match::new(text, 11, 12); // start is out of bounds
    let _ = m.as_str();
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_as_str_end_out_of_bounds() {
    let text = "hello world";
    let m = Match::new(text, 10, 15); // end is out of bounds
    let _ = m.as_str();
}

#[test]
fn test_as_str_zero_length_match() {
    let text = "hello world";
    let m = Match::new(text, 5, 5);
    assert_eq!(m.as_str(), ""); // zero-length match
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_as_str_start_greater_than_end() {
    let text = "hello world";
    let m = Match::new(text, 6, 5); // start is greater than end
    let _ = m.as_str();
}

#[test]
fn test_as_str_full_string_match() {
    let text = "hello world";
    let m = Match::new(text, 0, text.len());
    assert_eq!(m.as_str(), "hello world"); // match the full string
}

