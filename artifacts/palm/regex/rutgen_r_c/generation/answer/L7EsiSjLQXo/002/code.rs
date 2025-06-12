// Answer 0

#[test]
fn test_position_haystack_equals_needle() {
    let needle = b"abc";
    let haystack = b"abc";
    let result = position(needle, haystack);
    assert_eq!(result, Some(0));
}

#[test]
fn test_position_haystack_longer_than_needle() {
    let needle = b"abc";
    let haystack = b"abcdef";
    let result = position(needle, haystack);
    assert_eq!(result, Some(0));
}

#[test]
fn test_position_haystack_no_match() {
    let needle = b"abc";
    let haystack = b"defgh";
    let result = position(needle, haystack);
    assert_eq!(result, None);
}

#[test]
fn test_position_haystack_with_padding() {
    let needle = b"abc";
    let haystack = b"  abc ";
    let result = position(needle, haystack);
    assert_eq!(result, Some(2));
}

#[test]
fn test_position_needle_longer_than_haystack() {
    let needle = b"abc";
    let haystack = b"ab";
    let result = position(needle, haystack);
    assert_eq!(result, None);
}

#[test]
fn test_position_haystack_empty() {
    let needle = b"abc";
    let haystack = b"";
    let result = position(needle, haystack);
    assert_eq!(result, None);
}

#[test]
fn test_position_haystack_equals_empty() {
    let needle = b"";
    let haystack = b"";
    let result = position(needle, haystack);
    assert_eq!(result, Some(0));
}

#[test]
fn test_position_needle_empty() {
    let needle = b"";
    let haystack = b"abc";
    let result = position(needle, haystack);
    assert_eq!(result, Some(0));
}

