// Answer 0

#[test]
fn test_position_exact_match() {
    let needle = b"abc";
    let haystack = b"abc";
    assert_eq!(position(needle, haystack), Some(0));
}

#[test]
fn test_position_no_match_equal_lengths() {
    let needle = b"abc";
    let haystack = b"def"; // haystack length == needle length, but different content
    assert_eq!(position(needle, haystack), None);
}

#[test]
fn test_position_no_match_shorter_haystack() {
    let needle = b"abc";
    let haystack = b"de"; // haystack shorter than needle
    assert_eq!(position(needle, haystack), None);
} 

#[test]
fn test_position_empty_haystack() {
    let needle = b"abc";
    let haystack = b""; // empty haystack
    assert_eq!(position(needle, haystack), None);
}

#[test]
fn test_position_haystack_starting_with_needle() {
    let needle = b"abc";
    let haystack = b"abcxyz"; // haystack contains needle at the beginning
    assert_eq!(position(needle, haystack), Some(0));
}

#[test]
fn test_position_haystack_not_starting_with_needle() {
    let needle = b"abc";
    let haystack = b"xyzabc"; // haystack contains needle but not at the beginning
    assert_eq!(position(needle, haystack), Some(3));
}

