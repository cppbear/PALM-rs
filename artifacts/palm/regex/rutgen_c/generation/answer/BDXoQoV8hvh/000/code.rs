// Answer 0

#[test]
fn test_match_end() {
    let haystack: &[u8] = b"hello, world";
    let start = 7;
    let end = 12;
    
    let m = Match::new(haystack, start, end);
    
    assert_eq!(m.end(), end);
}

#[test]
fn test_match_end_boundary() {
    let haystack: &[u8] = b"";
    let start = 0;
    let end = 0;
    
    let m = Match::new(haystack, start, end);
    
    assert_eq!(m.end(), end);
}

#[test]
fn test_match_end_negative_case() {
    let haystack: &[u8] = b"match";
    let start = 0;
    let end = 5;
    
    let m = Match::new(haystack, start, end);
    
    assert_ne!(m.end(), 6);
}

