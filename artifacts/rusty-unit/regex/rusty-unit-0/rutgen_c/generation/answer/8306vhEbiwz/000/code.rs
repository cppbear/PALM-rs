// Answer 0

#[test]
fn test_match_as_bytes() {
    let text: &[u8] = b"Hello, world!";
    let start: usize = 7;
    let end: usize = 12;
    let matched = Match::new(text, start, end);
    
    assert_eq!(matched.as_bytes(), b"world");
}

#[test]
fn test_match_as_bytes_empty() {
    let text: &[u8] = b"";
    let start: usize = 0;
    let end: usize = 0;
    let matched = Match::new(text, start, end);
    
    assert_eq!(matched.as_bytes(), b"");
}

