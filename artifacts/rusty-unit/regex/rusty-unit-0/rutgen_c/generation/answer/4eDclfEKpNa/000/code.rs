// Answer 0

#[test]
fn test_is_word_byte() {
    assert_eq!(is_word_byte(b'_'), true);
    assert_eq!(is_word_byte(b'0'), true);
    assert_eq!(is_word_byte(b'9'), true);
    assert_eq!(is_word_byte(b'a'), true);
    assert_eq!(is_word_byte(b'z'), true);
    assert_eq!(is_word_byte(b'A'), true);
    assert_eq!(is_word_byte(b'Z'), true);
    
    assert_eq!(is_word_byte(b' '), false);
    assert_eq!(is_word_byte(b'*'), false);
    assert_eq!(is_word_byte(b'-'), false);
    assert_eq!(is_word_byte(b'&'), false);
    assert_eq!(is_word_byte(b'@'), false);
}

