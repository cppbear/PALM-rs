// Answer 0

#[test]
fn test_is_word_byte() {
    assert_eq!(is_word_byte(b'_'), true); // constraint: c matches b'_' is true
    assert_eq!(is_word_byte(b'0'), false); // constraint: c matches b'0' ... b'9' is false
    assert_eq!(is_word_byte(b'a'), true); // constraint: c matches b'a' ... b'z' is true
    assert_eq!(is_word_byte(b'z'), true); // constraint: c matches b'a' ... b'z' is true
    assert_eq!(is_word_byte(b'A'), true); // constraint: c matches b'A' ... b'Z' is true
    assert_eq!(is_word_byte(b'Z'), true); // constraint: c matches b'A' ... b'Z' is true
    assert_eq!(is_word_byte(b'_'), true); // constraint: c matches b'_' is true
    assert_eq!(is_word_byte(b'9'), false); // constraint: c matches b'0' ... b'9' is false
}

