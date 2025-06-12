// Answer 0

#[test]
fn test_should_use_min_length() {
    let pattern: &[u8] = b"abcdefghi"; // length == MIN_LEN
    assert_eq!(should_use(pattern), false);
}

#[test]
fn test_should_use_just_above_min_length() {
    let pattern: &[u8] = b"abcdefghij"; // length > MIN_LEN
    assert_eq!(should_use(pattern), true);
}

#[test]
fn test_should_use_pattern_with_rarer_bytes() {
    let pattern: &[u8] = b"abcdefghijk"; // length > MIN_LEN
    // Assuming freq_rank of 'a' < cutoff. This will trigger the false condition on ranks.
    fn freq_rank(c: u8) -> usize {
        match c {
            b'a' => 149, // Rare byte
            _ => 200,    // Other bytes are more common
        }
    }
    assert_eq!(should_use(pattern), false);
}

#[test]
fn test_should_use_pattern_with_frequent_bytes() {
    let pattern: &[u8] = b"abcdefghij"; // length > MIN_LEN
    // All bytes are frequent enough.
    fn freq_rank(c: u8) -> usize {
        200 // All bytes are frequent
    }
    assert_eq!(should_use(pattern), true);
}

