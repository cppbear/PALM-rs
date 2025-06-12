// Answer 0

#[test]
fn test_char_len() {
    let pattern = vec![b'a', b'b', b'c', b'd', b'e'];
    let freqy_packed = FreqyPacked {
        pat: pattern.clone(),
        char_len: 5,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    assert_eq!(freqy_packed.char_len(), 5);

    let empty_pattern = vec![];
    let empty_freqy_packed = FreqyPacked {
        pat: empty_pattern.clone(),
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };
    assert_eq!(empty_freqy_packed.char_len(), 0);
}

