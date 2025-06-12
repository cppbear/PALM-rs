// Answer 0

#[test]
fn test_len_non_empty() {
    let packed = FreqyPacked {
        pat: vec![1, 2, 3, 4],
        char_len: 4,
        rare1: 1,
        rare1i: 0,
        rare2: 2,
        rare2i: 1,
    };
    assert_eq!(packed.len(), 4);
}

#[test]
fn test_len_empty() {
    let packed = FreqyPacked {
        pat: vec![],
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };
    assert_eq!(packed.len(), 0);
}

