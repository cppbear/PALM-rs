// Answer 0

#[test]
fn test_position_equal_lengths_not_matched() {
    let needle: &[u8] = b"abc";
    let haystack: &[u8] = b"def";

    position(needle, haystack);
}

#[test]
fn test_position_equal_lengths_first_byte_different() {
    let needle: &[u8] = b"xyz";
    let haystack: &[u8] = b"abc";

    position(needle, haystack);
}

#[test]
fn test_position_equal_lengths_non_matching_sequence() {
    let needle: &[u8] = b"ghi";
    let haystack: &[u8] = b"jkl";

    position(needle, haystack);
}

#[test]
fn test_position_equal_lengths_different_content() {
    let needle: &[u8] = b"123";
    let haystack: &[u8] = b"456";

    position(needle, haystack);
}

#[test]
fn test_position_equal_length_different_first_last() {
    let needle: &[u8] = b"abc";
    let haystack: &[u8] = b"cba";

    position(needle, haystack);
}

