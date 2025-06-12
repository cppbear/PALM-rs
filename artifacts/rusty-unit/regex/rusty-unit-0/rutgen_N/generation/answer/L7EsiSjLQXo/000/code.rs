// Answer 0

#[test]
fn test_position_found_at_start() {
    let needle = b"hello";
    let haystack = b"hello world";
    assert_eq!(position(needle, haystack), Some(0));
}

#[test]
fn test_position_found_in_middle() {
    let needle = b"world";
    let haystack = b"hello world";
    assert_eq!(position(needle, haystack), Some(6));
}

#[test]
fn test_position_not_found() {
    let needle = b"test";
    let haystack = b"hello world";
    assert_eq!(position(needle, haystack), None);
}

#[test]
fn test_position_empty_haystack() {
    let needle = b"test";
    let haystack: &[u8] = b"";
    assert_eq!(position(needle, haystack), None);
}

#[test]
fn test_position_empty_needle() {
    let needle: &[u8] = b"";
    let haystack = b"hello world";
    assert_eq!(position(needle, haystack), Some(0));
}

#[test]
fn test_position_haystack_shorter_than_needle() {
    let needle = b"hello";
    let haystack = b"hi";
    assert_eq!(position(needle, haystack), None);
}

#[test]
fn test_position_found_at_end() {
    let needle = b"world";
    let haystack = b"hello world";
    assert_eq!(position(needle, &haystack[5..]), Some(5));
}

