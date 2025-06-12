// Answer 0

#[test]
fn test_incomplete_utf8_sequence() {
    let input = &[0b1110_0000, 0b10_000000, 0b10_000000]; // valid prefix but incomplete
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_invalid_utf8_sequence_with_surrogate() {
    let input = &[0b1110_0000, 0b10_000000, 0b10_000000, 0b11_111111]; // Surrogate codepoint
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_correct_but_non_shortest_sequence() {
    let input = &[0b1110_0000, 0b10_000001, 0b10_000000]; // valid but not shortest UTF-8
    let result = decode_utf8(input);
    assert_eq!(result, None);
}

#[test]
fn test_valid_utf8_sequence() {
    let input = &[0b1110_0001, 0b1000_0001, 0b1000_0000]; // represents valid UTF-8 for U+2010
    let result = decode_utf8(input);
    assert_eq!(result, Some(('\u{2010}', 3)));
}

