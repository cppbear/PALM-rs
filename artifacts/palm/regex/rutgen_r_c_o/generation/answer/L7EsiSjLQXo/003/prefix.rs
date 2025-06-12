// Answer 0

#[test]
fn test_position_haystack_empty() {
    let needle: &[u8] = &[1];
    let haystack: &[u8] = &[];
    position(needle, haystack);
}

#[test]
fn test_position_haystack_shorter_than_needle() {
    let needle: &[u8] = &[1];
    let haystack: &[u8] = &[2];
    position(needle, haystack);
}

#[test]
fn test_position_haystack_equal_length_to_needle_no_match() {
    let needle: &[u8] = &[1];
    let haystack: &[u8] = &[2];
    position(needle, haystack);
}

