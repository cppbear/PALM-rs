// Answer 0

#[derive(Debug)]
struct Match<'t> {
    text: &'t str,
    start: usize,
    end: usize,
}

#[test]
fn test_new_match() {
    let haystack = "Hello, world!";
    let start = 0;
    let end = 5;
    let m = new(haystack, start, end);
    assert_eq!(m.text, "Hello, world!");
    assert_eq!(m.start, 0);
    assert_eq!(m.end, 5);
}

#[test]
fn test_new_match_empty_string() {
    let haystack = "";
    let start = 0;
    let end = 0;
    let m = new(haystack, start, end);
    assert_eq!(m.text, "");
    assert_eq!(m.start, 0);
    assert_eq!(m.end, 0);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_new_match_out_of_bounds_start() {
    let haystack = "test string";
    let start = 15; // Out of bounds
    let end = 20;   // Out of bounds
    let _m = new(haystack, start, end);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_new_match_out_of_bounds_end() {
    let haystack = "test string";
    let start = 5;
    let end = 20; // Out of bounds
    let _m = new(haystack, start, end);
}

#[test]
fn test_new_match_equal_start_end() {
    let haystack = "Boundary test";
    let start = 9;
    let end = 9; // Start equals end
    let m = new(haystack, start, end);
    assert_eq!(m.text, "Boundary test");
    assert_eq!(m.start, 9);
    assert_eq!(m.end, 9);
}

