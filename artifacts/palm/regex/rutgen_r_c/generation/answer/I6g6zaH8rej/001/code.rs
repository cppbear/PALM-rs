// Answer 0

#[test]
fn test_new_match_valid() {
    let haystack: &[u8] = b"hello, world";
    let start = 0;
    let end = 5;
    let m = new(haystack, start, end);
    assert_eq!(m.start(), start);
    assert_eq!(m.end(), end);
    assert_eq!(m.as_bytes(), b"hello");
}

#[test]
#[should_panic]
fn test_new_match_start_greater_than_end() {
    let haystack: &[u8] = b"hello, world";
    let start = 5;
    let end = 0;
    new(haystack, start, end);
}

#[test]
#[should_panic]
fn test_new_match_start_out_of_bounds() {
    let haystack: &[u8] = b"hello, world";
    let start = 15; // Out of bounds
    let end = 20; // Out of bounds
    new(haystack, start, end);
}

#[test]
fn test_new_match_empty_haystack() {
    let haystack: &[u8] = b"";
    let start = 0;
    let end = 0; // Valid match on empty haystack
    let m = new(haystack, start, end);
    assert_eq!(m.start(), start);
    assert_eq!(m.end(), end);
    assert_eq!(m.as_bytes(), b"");
}

#[test]
fn test_new_match_single_character() {
    let haystack: &[u8] = b"a";
    let start = 0;
    let end = 1; // Match the single character
    let m = new(haystack, start, end);
    assert_eq!(m.start(), start);
    assert_eq!(m.end(), end);
    assert_eq!(m.as_bytes(), b"a");
}

#[test]
fn test_new_match_full_haystack() {
    let haystack: &[u8] = b"example";
    let start = 0;
    let end = 7; // Full match of the haystack
    let m = new(haystack, start, end);
    assert_eq!(m.start(), start);
    assert_eq!(m.end(), end);
    assert_eq!(m.as_bytes(), b"example");
}

