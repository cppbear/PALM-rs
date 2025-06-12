// Answer 0

#[test]
fn test_approximate_size_with_non_empty_pattern() {
    let pattern = vec![b'a', b'b', b'c', b'd'];
    let freqy = FreqyPacked {
        pat: pattern,
        char_len: 4,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    assert_eq!(freqy.approximate_size(), 4);
}

#[test]
fn test_approximate_size_with_empty_pattern() {
    let pattern = vec![];
    let freqy = FreqyPacked {
        pat: pattern,
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };
    assert_eq!(freqy.approximate_size(), 0);
}

