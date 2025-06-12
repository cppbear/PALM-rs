// Answer 0

#[test]
fn test_decode_utf8_valid_single_byte() {
    let input = [0x41]; // 'A'
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('A', 1)));
}

#[test]
fn test_decode_utf8_valid_double_byte() {
    let input = [0xC2, 0xA9]; // '©'
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('©', 2)));
}

#[test]
fn test_decode_utf8_valid_triple_byte() {
    let input = [0xE2, 0x82, 0xAC]; // '€'
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('€', 3)));
}

#[test]
fn test_decode_utf8_valid_quadruple_byte() {
    let input = [0xF0, 0x9F, 0x92, 0xA9]; // '💩'
    let result = decode_utf8(&input);
    assert_eq!(result, Some(('💩', 4)));
}

#[test]
fn test_decode_utf8_invalid_too_short() {
    let input = [0xC2]; // Incomplete UTF-8 sequence
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_overlong() {
    let input = [0xC0, 0xAF]; // Overlong encoding of '/'
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_surrogate_codepoint() {
    let input = [0xED, 0x9F, 0xBF]; // Surrogate codepoint (U+DFFF)
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_invalid_continuation_byte() {
    let input = [0xE2, 0x28, 0xA1]; // Invalid continuation byte
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_decode_utf8_empty_input() {
    let input: &[u8] = &[];
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

