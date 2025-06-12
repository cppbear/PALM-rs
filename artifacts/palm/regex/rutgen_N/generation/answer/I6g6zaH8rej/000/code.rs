// Answer 0

#[derive(Debug)]
struct Match<'t> {
    text: &'t [u8],
    start: usize,
    end: usize,
}

#[test]
fn test_new_match_valid() {
    let haystack: &[u8] = b"hello, world";
    let start = 0;
    let end = 5;
    let m = new(haystack, start, end);
    assert_eq!(m.text, haystack);
    assert_eq!(m.start, start);
    assert_eq!(m.end, end);
}

#[test]
#[should_panic]
fn test_new_match_invalid_start() {
    let haystack: &[u8] = b"hello, world";
    let start = 10;
    let end = 5;
    let _m = new(haystack, start, end); // Should panic due to invalid offsets
}

#[test]
#[should_panic]
fn test_new_match_invalid_end() {
    let haystack: &[u8] = b"hello, world";
    let start = 5;
    let end = 15;
    let _m = new(haystack, start, end); // Should panic due to invalid offsets
}

