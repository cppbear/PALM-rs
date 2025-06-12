// Answer 0

#[test]
fn test_position_equal_single_byte() {
    let needle: &[u8] = b"A";
    let haystack: &[u8] = b"A";
    position(needle, haystack);
}

#[test]
fn test_position_empty_needle() {
    let needle: &[u8] = b"";
    let haystack: &[u8] = b"A";
    position(needle, haystack);
}

#[test]
fn test_position_single_byte_haystack() {
    let needle: &[u8] = b"A";
    let haystack: &[u8] = b"A";
    position(needle, haystack);
}

