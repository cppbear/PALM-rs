// Answer 0

#[test]
fn test_position_with_empty_haystack() {
    let needle: &[u8] = b"abc";
    let haystack: &[u8] = b"";
    assert_eq!(position(needle, haystack), None);
}

#[test]
fn test_position_with_short_haystack() {
    let needle: &[u8] = b"longer";
    let haystack: &[u8] = b"short";
    assert_eq!(position(needle, haystack), None);
}

#[test]
fn test_position_with_haystack_equal_to_needle() {
    let needle: &[u8] = b"xyz";
    let haystack: &[u8] = b"xyz";
    assert_eq!(position(needle, haystack), None);
}

#[test]
fn test_position_with_same_length_haystack_and_needle_no_match() {
    let needle: &[u8] = b"abc";
    let haystack: &[u8] = b"def";
    assert_eq!(position(needle, haystack), None);
}

