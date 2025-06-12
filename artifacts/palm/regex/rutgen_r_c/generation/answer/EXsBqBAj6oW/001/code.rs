// Answer 0

#[test]
fn test_match_start() {
    let haystack: &[u8] = b"Hello, world!";
    let match_start = Match::new(haystack, 7, 12);
    
    assert_eq!(match_start.start(), 7);
}

#[test]
fn test_match_start_edge_case_zero() {
    let haystack: &[u8] = b"";
    let match_start = Match::new(haystack, 0, 0);
    
    assert_eq!(match_start.start(), 0);
}

#[test]
fn test_match_start_full_length() {
    let haystack: &[u8] = b"Test string.";
    let match_start = Match::new(haystack, 0, 12);
    
    assert_eq!(match_start.start(), 0);
}

#[test]
fn test_match_start_at_end() {
    let haystack: &[u8] = b"Another example";
    let match_start = Match::new(haystack, 15, 15);
    
    assert_eq!(match_start.start(), 15);
}

