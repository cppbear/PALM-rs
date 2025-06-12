// Answer 0

#[test]
fn test_char_len_non_empty_pattern() {
    let pattern = vec![b'a', b'b', b'c'];
    let freqy = FreqyPacked {
        pat: pattern.clone(),
        char_len: pattern.len(),
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    assert_eq!(freqy.char_len(), 3);
}

#[test]
fn test_char_len_empty_pattern() {
    let pattern = Vec::new();
    let freqy = FreqyPacked::empty();
    assert_eq!(freqy.char_len(), 0);
}

#[test]
fn test_char_len_single_byte_pattern() {
    let pattern = vec![b'x'];
    let freqy = FreqyPacked {
        pat: pattern.clone(),
        char_len: pattern.len(),
        rare1: b'x',
        rare1i: 0,
        rare2: b'x',
        rare2i: 0,
    };
    assert_eq!(freqy.char_len(), 1);
}

#[test]
fn test_char_len_utf8_pattern_with_incomplete_sequence() {
    let pattern = vec![0xe2, 0x82, 0xac, 0xe2, 0x82]; // U+20AC (EURO SIGN) incomplete
    let freqy = FreqyPacked {
        pat: pattern.clone(),
        char_len: 4, // Assuming we want to count full characters
        rare1: 0xe2,
        rare1i: 0,
        rare2: 0x82,
        rare2i: 1,
    };
    assert_eq!(freqy.char_len(), 4);
}

