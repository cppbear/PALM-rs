// Answer 0

#[test]
fn test_new_match_valid() {
    let haystack = "hello world";
    let start = 0;
    let end = 5;
    let m = Match::new(haystack, start, end);
    assert_eq!(m.as_str(), "hello");
    assert_eq!(m.start(), 0);
    assert_eq!(m.end(), 5);
}

#[test]
#[should_panic]
fn test_new_match_start_greater_than_end() {
    let haystack = "test string";
    let start = 5;
    let end = 3;
    Match::new(haystack, start, end);
}

#[test]
#[should_panic]
fn test_new_match_start_out_of_bounds() {
    let haystack = "example";
    let start = 10; // out of bounds
    let end = 15;  // also out of bounds
    Match::new(haystack, start, end);
}

#[test]
#[should_panic]
fn test_new_match_end_out_of_bounds() {
    let haystack = "another test";
    let start = 8;
    let end = 20; // out of bounds
    Match::new(haystack, start, end);
}

