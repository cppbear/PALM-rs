// Answer 0

#[test]
fn test_new_match_valid_bounds() {
    let haystack: &[u8] = b"Hello, world!";
    let start = 0;
    let end = 5;
    let result = new(haystack, start, end);
    assert_eq!(result.text, haystack);
    assert_eq!(result.start, start);
    assert_eq!(result.end, end);
}

#[test]
#[should_panic]
fn test_new_match_start_out_of_bounds() {
    let haystack: &[u8] = b"Hello, world!";
    let start = 15;
    let end = 20;
    new(haystack, start, end);
}

#[test]
#[should_panic]
fn test_new_match_end_out_of_bounds() {
    let haystack: &[u8] = b"Hello, world!";
    let start = 5;
    let end = 20;
    new(haystack, start, end);
}

#[test]
#[should_panic]
fn test_new_match_start_greater_than_end() {
    let haystack: &[u8] = b"Hello, world!";
    let start = 5;
    let end = 2;
    new(haystack, start, end);
}

#[test]
fn test_new_match_zero_length() {
    let haystack: &[u8] = b"";
    let start = 0;
    let end = 0;
    let result = new(haystack, start, end);
    assert_eq!(result.text, haystack);
    assert_eq!(result.start, start);
    assert_eq!(result.end, end);
}

#[test]
fn test_new_match_entire_haystack() {
    let haystack: &[u8] = b"Hello, world!";
    let start = 0;
    let end = haystack.len();
    let result = new(haystack, start, end);
    assert_eq!(result.text, haystack);
    assert_eq!(result.start, start);
    assert_eq!(result.end, end);
}

