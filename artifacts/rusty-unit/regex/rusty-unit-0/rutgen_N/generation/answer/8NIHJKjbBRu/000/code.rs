// Answer 0

#[test]
fn test_next_utf8_single_byte_character() {
    let text = b"hello";
    assert_eq!(next_utf8(text, 0), 1);
    assert_eq!(next_utf8(text, 1), 2);
    assert_eq!(next_utf8(text, 2), 3);
    assert_eq!(next_utf8(text, 3), 4);
    assert_eq!(next_utf8(text, 4), 5);
}

#[test]
fn test_next_utf8_two_byte_character() {
    let text = b"\xc2\xa9"; // © character
    assert_eq!(next_utf8(text, 0), 2);
    assert_eq!(next_utf8(text, 1), 2); // 1 is second byte, must point to next valid position
}

#[test]
fn test_next_utf8_three_byte_character() {
    let text = b"\xe2\x9c\x94"; // ✓ character
    assert_eq!(next_utf8(text, 0), 3);
    assert_eq!(next_utf8(text, 1), 3); // 1 is part of sequence, must point to next valid position
    assert_eq!(next_utf8(text, 2), 3); // 2 is part of sequence, must point to next valid position
}

#[test]
fn test_next_utf8_four_byte_character() {
    let text = b"\xf0\x9f\x92\xa9"; // 💩 character
    assert_eq!(next_utf8(text, 0), 4);
    assert_eq!(next_utf8(text, 1), 4); // 1 is part of sequence, must point to next valid position
    assert_eq!(next_utf8(text, 2), 4); // 2 is part of sequence, must point to next valid position
    assert_eq!(next_utf8(text, 3), 4); // 3 is part of sequence, must point to next valid position
}

#[test]
fn test_next_utf8_out_of_bounds() {
    let text = b"hello";
    assert_eq!(next_utf8(text, 5), 6); // beyond the last byte
    assert_eq!(next_utf8(text, 6), 7); // well beyond the last byte
}

#[test]
fn test_next_utf8_non_utf8_sequence() {
    let text = b"\xff\xff"; // Invalid bytes following 0xFF
    assert_eq!(next_utf8(text, 0), 1);
    assert_eq!(next_utf8(text, 1), 2); // Moving to non-UTF-8 data should point to the next byte
}

#[test]
fn test_next_utf8_valid_and_invalid() {
    let text = b"hello \xc2\xa9world \xff"; // Mixed content
    assert_eq!(next_utf8(text, 5), 7); // After 'hello ' should be at the start of ©
    assert_eq!(next_utf8(text, 6), 7); // Should point to next valid after ©
    assert_eq!(next_utf8(text, 7), 8); // Should point to 'w'
    assert_eq!(next_utf8(text, 8), 9); // Should point to 'o'
    assert_eq!(next_utf8(text, 9), 10); // Should point to 'r'
    assert_eq!(next_utf8(text, 10), 11); // Should point to 'l'
    assert_eq!(next_utf8(text, 11), 12); // Should point to 'd'
    assert_eq!(next_utf8(text, 12), 13); // Should point to the end
}

