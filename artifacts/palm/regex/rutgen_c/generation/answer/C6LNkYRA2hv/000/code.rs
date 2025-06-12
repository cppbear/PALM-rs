// Answer 0

#[test]
fn test_is_suffix_true() {
    let freqy = FreqyPacked {
        pat: vec![1, 2, 3],
        char_len: 3,
        rare1: 1,
        rare1i: 0,
        rare2: 2,
        rare2i: 1,
    };
    let text = vec![0, 0, 1, 2, 3];
    assert!(freqy.is_suffix(&text));
}

#[test]
fn test_is_suffix_false_shorter_text() {
    let freqy = FreqyPacked {
        pat: vec![1, 2, 3],
        char_len: 3,
        rare1: 1,
        rare1i: 0,
        rare2: 2,
        rare2i: 1,
    };
    let text = vec![1, 2];
    assert!(!freqy.is_suffix(&text));
}

#[test]
fn test_is_suffix_false_different_suffix() {
    let freqy = FreqyPacked {
        pat: vec![1, 2, 3],
        char_len: 3,
        rare1: 1,
        rare1i: 0,
        rare2: 2,
        rare2i: 1,
    };
    let text = vec![0, 1, 2, 4];
    assert!(!freqy.is_suffix(&text));
}

#[test]
fn test_is_suffix_empty_pattern() {
    let freqy = FreqyPacked {
        pat: vec![],
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };
    let text = vec![0, 1, 2, 3];
    assert!(freqy.is_suffix(&text));
}

#[test]
fn test_is_suffix_empty_text() {
    let freqy = FreqyPacked {
        pat: vec![1],
        char_len: 1,
        rare1: 1,
        rare1i: 0,
        rare2: 2,
        rare2i: 1,
    };
    let text: Vec<u8> = vec![];
    assert!(!freqy.is_suffix(&text));
}

