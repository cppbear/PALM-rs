// Answer 0

#[test]
fn test_position_equal_length_haystack_needle() {
    let needle: &[u8] = b"test";
    let haystack: &[u8] = b"test";
    let result = position(needle, haystack);
    assert_eq!(result, Some(0));
}

#[test]
fn test_position_equal_length_haystack_needle_not_found() {
    let needle: &[u8] = b"abcd";
    let haystack: &[u8] = b"efgh";
    let result = position(needle, haystack);
    assert_eq!(result, None);
}

#[test]
fn test_position_single_byte_match() {
    let needle: &[u8] = b"a";
    let haystack: &[u8] = b"a";
    let result = position(needle, haystack);
    assert_eq!(result, Some(0));
}

#[test]
fn test_position_single_byte_no_match() {
    let needle: &[u8] = b"a";
    let haystack: &[u8] = b"b";
    let result = position(needle, haystack);
    assert_eq!(result, None);
}

#[test]
fn test_position_haystack_with_repeated_chars() {
    let needle: &[u8] = b"x";
    let haystack: &[u8] = b"xxxx";
    let result = position(needle, haystack);
    assert_eq!(result, Some(0));
}

#[test]
#[should_panic]
fn test_position_panic_condition() {
    let needle: &[u8] = b"test";
    let haystack: &[u8] = b"te"; // haystack shorter than needle, should not panic in normal use
    let _ = position(needle, haystack);
}

