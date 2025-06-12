// Answer 0

#[test]
fn test_decode_utf8_empty_slice() {
    let result = decode_utf8(&[]);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_utf8_sequence() {
    let result = decode_utf8(&[0xC0, 0xAF]); // Incomplete 2-byte sequence
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_overflow_sequence() {
    let result = decode_utf8(&[0xF4, 0x90, 0x80, 0x80]); // Codepoint above U+10FFFF
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_surrogate_codepoint() {
    let result = decode_utf8(&[0xED, 0xA0, 0x80]); // Surrogate codepoint 0xD800
    assert_eq!(result, None);
}

