// Answer 0

#[test]
fn test_position_found_at_start() {
    let needle = b"abc";
    let haystack = b"abcdef";
    assert_eq!(position(needle, haystack), Some(0));
}

#[test]
fn test_position_found_in_middle() {
    let needle = b"def";
    let haystack = b"abcdef";
    assert_eq!(position(needle, haystack), Some(3));
}

#[test]
fn test_position_not_found() {
    let needle = b"xyz";
    let haystack = b"abcdef";
    assert_eq!(position(needle, haystack), None);
}

#[test]
fn test_position_needle_too_long() {
    let needle = b"abcdefg";
    let haystack = b"abcdef";
    assert_eq!(position(needle, haystack), None);
}

#[test]
fn test_position_empty_needle() {
    let needle = b"";
    let haystack = b"abcdef";
    assert_eq!(position(needle, haystack), Some(0));
}

#[test]
fn test_position_empty_haystack() {
    let needle = b"abc";
    let haystack = b"";
    assert_eq!(position(needle, haystack), None);
}

#[test]
fn test_position_needle_empty_haystack_empty() {
    let needle = b"";
    let haystack = b"";
    assert_eq!(position(needle, haystack), Some(0));
}

