// Answer 0

#[test]
fn test_new_match_valid() {
    let haystack: &[u8] = b"Hello, world!";
    let start = 0;
    let end = 5;

    let result = Match::new(haystack, start, end);
    assert_eq!(result.start(), start);
    assert_eq!(result.end(), end);
    assert_eq!(result.as_bytes(), &haystack[start..end]);
}

#[test]
fn test_new_match_empty() {
    let haystack: &[u8] = b"";
    let start = 0;
    let end = 0;

    let result = Match::new(haystack, start, end);
    assert_eq!(result.start(), start);
    assert_eq!(result.end(), end);
    assert_eq!(result.as_bytes(), &haystack[start..end]);
}

#[test]
fn test_new_match_full_range() {
    let haystack: &[u8] = b"Full range match";
    let start = 0;
    let end = haystack.len();

    let result = Match::new(haystack, start, end);
    assert_eq!(result.start(), start);
    assert_eq!(result.end(), end);
    assert_eq!(result.as_bytes(), &haystack[start..end]);
}

#[should_panic]
fn test_new_match_invalid_end() {
    let haystack: &[u8] = b"Hello";
    let start = 1;
    let end = 10; // This should panic because it's out of bounds

    let _result = Match::new(haystack, start, end);
}

#[should_panic]
fn test_new_match_invalid_start_end() {
    let haystack: &[u8] = b"Hello";
    let start = 5;
    let end = 1; // This should panic because start is greater than end

    let _result = Match::new(haystack, start, end);
}

