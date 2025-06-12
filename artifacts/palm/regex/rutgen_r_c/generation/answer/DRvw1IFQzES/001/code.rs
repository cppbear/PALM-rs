// Answer 0

#[test]
fn test_match_start() {
    // Test with a simple match
    let haystack = "Hello, world!";
    let start = 7;
    let end = 12;
    let m = Match::new(haystack, start, end);
    assert_eq!(m.start(), 7);

    // Test with a match at the beginning of the string
    let m = Match::new(haystack, 0, 5);
    assert_eq!(m.start(), 0);
    
    // Test with a match at the end of the string
    let m = Match::new(haystack, 7, 12);
    assert_eq!(m.start(), 7);

    // Test with an empty string
    let m = Match::new("", 0, 0);
    assert_eq!(m.start(), 0);
    
    // Test with a match covering the entire string
    let m = Match::new(haystack, 0, haystack.len());
    assert_eq!(m.start(), 0);
}

