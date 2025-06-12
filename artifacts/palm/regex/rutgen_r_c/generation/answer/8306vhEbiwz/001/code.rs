// Answer 0

#[test]
fn test_match_as_bytes_valid_range() {
    let text: &[u8] = b"Hello, world!";
    let m = Match::new(text, 0, 5);
    let result = m.as_bytes();
    assert_eq!(result, b"Hello");
}

#[test]
fn test_match_as_bytes_full_range() {
    let text: &[u8] = b"Hello, world!";
    let m = Match::new(text, 0, text.len());
    let result = m.as_bytes();
    assert_eq!(result, b"Hello, world!");
}

#[test]
#[should_panic]
fn test_match_as_bytes_out_of_bounds_start() {
    let text: &[u8] = b"Hello, world!";
    let m = Match::new(text, 5, 15);
    let _ = m.as_bytes();  // This should panic due to out of bounds
}

#[test]
#[should_panic]
fn test_match_as_bytes_negative_start() {
    let text: &[u8] = b"Hello, world!";
    let m = Match::new(text, usize::MAX, 5);
    let _ = m.as_bytes();  // This should panic due to negative start index
}

#[test]
#[should_panic]
fn test_match_as_bytes_end_before_start() {
    let text: &[u8] = b"Hello, world!";
    let m = Match::new(text, 5, 0);
    let _ = m.as_bytes();  // This should panic due to end before start
}

#[test]
#[should_panic]
fn test_match_as_bytes_end_out_of_bounds() {
    let text: &[u8] = b"Hello, world!";
    let m = Match::new(text, 2, 20);
    let _ = m.as_bytes();  // This should panic due to end out of bounds
}

