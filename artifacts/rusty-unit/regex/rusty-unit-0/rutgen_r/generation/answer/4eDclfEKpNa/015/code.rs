// Answer 0

#[test]
fn test_is_word_byte_constraints() {
    // Input that matches the constraint of being b'_'
    let c: u8 = b'_';
    assert_eq!(is_word_byte(c), true);

    // Input that matches the constraint of being not in the range b'0' ... b'9'
    let c: u8 = b':';
    assert_eq!(is_word_byte(c), false);

    // Input that matches the constraint of being not in the range b'a' ... b'z'
    let c: u8 = b'{' ;  // After 'z'
    assert_eq!(is_word_byte(c), false);

    // Input that matches the constraint of being not in the range b'A' ... b'Z'
    let c: u8 = b'[';  // Before 'A'
    assert_eq!(is_word_byte(c), false);
}

