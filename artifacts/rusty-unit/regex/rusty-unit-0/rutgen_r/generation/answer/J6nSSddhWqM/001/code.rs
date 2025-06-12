// Answer 0

#[test]
fn test_len_utf8_valid_chars() {
    struct CharWrapper(u32);

    let valid_char_a = CharWrapper(97); // 'a' has a UTF-8 length of 1
    let valid_char_grave_accent = CharWrapper(96); // '`' has a UTF-8 length of 1
    let valid_char_heart = CharWrapper(0x2764); // '❤' has a UTF-8 length of 3
    let valid_char_chinese = CharWrapper(0x4E2D); // '中' has a UTF-8 length of 3
    let valid_char_grin_face = CharWrapper(0x1F600); // '😀' has a UTF-8 length of 4

    assert_eq!(valid_char_a.len_utf8(), 1);
    assert_eq!(valid_char_grave_accent.len_utf8(), 1);
    assert_eq!(valid_char_heart.len_utf8(), 3);
    assert_eq!(valid_char_chinese.len_utf8(), 3);
    assert_eq!(valid_char_grin_face.len_utf8(), 4);
}

#[test]
fn test_len_utf8_invalid_chars() {
    struct CharWrapper(u32);

    let invalid_char_high_surrogate = CharWrapper(0xD800); // Invalid high surrogate, should return 0
    let invalid_char_high_surrogate_range = CharWrapper(0xDC00); // Invalid low surrogate, should return 0
    let invalid_char_out_of_range = CharWrapper(0x110000); // Out of valid range, should return 0

    assert_eq!(invalid_char_high_surrogate.len_utf8(), 0);
    assert_eq!(invalid_char_high_surrogate_range.len_utf8(), 0);
    assert_eq!(invalid_char_out_of_range.len_utf8(), 0);
}

#[test]
fn test_len_utf8_zero_case() {
    struct CharWrapper(u32);

    let zero_case = CharWrapper(0); // Invalid code point, should return 0

    assert_eq!(zero_case.len_utf8(), 0);
}

