// Answer 0

#[test]
fn test_match_start() {
    let text: &[u8] = b"Hello, world!";
    let match_instance = Match::new(text, 7, 12);
    assert_eq!(match_instance.start(), 7);
}

#[test]
fn test_match_start_boundary() {
    let text: &[u8] = b"";
    let match_instance = Match::new(text, 0, 0);
    assert_eq!(match_instance.start(), 0);
}

#[test]
fn test_match_start_full_length() {
    let text: &[u8] = b"abcd";
    let match_instance = Match::new(text, 0, 4);
    assert_eq!(match_instance.start(), 0);
}

#[test]
fn test_match_start_out_of_bounds() {
    let text: &[u8] = b"abc";
    let match_instance = Match::new(text, 3, 3);
    assert_eq!(match_instance.start(), 3);
}

