// Answer 0

#[test]
fn test_position_found_at_start() {
    let needle: &[u8] = b"abc";
    let haystack: &[u8] = b"abc";
    assert_eq!(position(needle, haystack), Some(0));
}

#[test]
fn test_position_found_in_middle() {
    let needle: &[u8] = b"abc";
    let haystack: &[u8] = b"xyzabc";
    assert_eq!(position(needle, &haystack[3..]), Some(0));
}

#[test]
fn test_position_found_at_end() {
    let needle: &[u8] = b"abc";
    let haystack: &[u8] = b"xyzabcxyz";
    assert_eq!(position(needle, &haystack[3..]), Some(0));
}

#[test]
fn test_position_not_found() {
    let needle: &[u8] = b"abc";
    let haystack: &[u8] = b"xyz";
    assert_eq!(position(needle, haystack), None);
}

#[test]
fn test_position_identical_needle_haystack() {
    let needle: &[u8] = b"abcd";
    let haystack: &[u8] = b"abcd";
    assert_eq!(position(needle, haystack), Some(0));
}

#[test]
fn test_position_empty_needle() {
    let needle: &[u8] = b"";
    let haystack: &[u8] = b"abc";
    assert_eq!(position(needle, haystack), Some(0));
}

