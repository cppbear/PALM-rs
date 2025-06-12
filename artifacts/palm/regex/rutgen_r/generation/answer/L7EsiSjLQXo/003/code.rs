// Answer 0

#[test]
fn test_position_haystack_shorter_than_needle() {
    let needle = b"longer";
    let haystack = b"short";
    let result = position(needle, haystack);
    assert_eq!(result, None);
}

#[test]
fn test_position_haystack_empty_needle_not_empty() {
    let needle = b"notempty";
    let haystack = b"";
    let result = position(needle, haystack);
    assert_eq!(result, None);
}

#[test]
fn test_position_haystack_empty_needle_empty() {
    let needle = b"";
    let haystack = b"";
    let result = position(needle, haystack);
    assert_eq!(result, None);
}

#[test]
fn test_position_haystack_equal_to_needle() {
    let needle = b"equal";
    let haystack = b"equal";
    let result = position(needle, haystack);
    assert_eq!(result, None);
}

