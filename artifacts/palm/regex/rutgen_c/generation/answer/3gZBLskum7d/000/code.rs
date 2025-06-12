// Answer 0

#[test]
fn test_new_match_valid() {
    let haystack = "Hello, world!";
    let start = 7;
    let end = 12;
    let m = Match::new(haystack, start, end);
    assert_eq!(m.start(), start);
    assert_eq!(m.end(), end);
    assert_eq!(m.as_str(), &haystack[start..end]);
}

#[test]
fn test_new_match_boundary() {
    let haystack = "Rust programming";
    let start = 0;
    let end = 4;
    let m = Match::new(haystack, start, end);
    assert_eq!(m.start(), start);
    assert_eq!(m.end(), end);
    assert_eq!(m.as_str(), &haystack[start..end]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_new_match_out_of_bounds_start() {
    let haystack = "Index out of bounds";
    let start = 25; // out of bounds
    let end = 29; // out of bounds
    Match::new(haystack, start, end);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_new_match_out_of_bounds_end() {
    let haystack = "Another test";
    let start = 5;
    let end = 15; // out of bounds
    Match::new(haystack, start, end);
}

#[test]
fn test_new_match_same_start_end() {
    let haystack = "Same start and end";
    let start = 5;
    let end = 5; // same value
    let m = Match::new(haystack, start, end);
    assert_eq!(m.start(), start);
    assert_eq!(m.end(), end);
    assert_eq!(m.as_str(), &haystack[start..end]);
}

