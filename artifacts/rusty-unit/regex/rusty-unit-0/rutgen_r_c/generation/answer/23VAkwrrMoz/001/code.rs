// Answer 0

#[test]
fn test_approximate_size_non_empty() {
    let pattern = FreqyPacked {
        pat: vec![1, 2, 3, 4, 5],
        char_len: 5,
        rare1: 1,
        rare1i: 0,
        rare2: 2,
        rare2i: 1,
    };
    assert_eq!(pattern.approximate_size(), 5 * std::mem::size_of::<u8>());
}

#[test]
fn test_approximate_size_empty() {
    let pattern = FreqyPacked {
        pat: vec![],
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };
    assert_eq!(pattern.approximate_size(), 0 * std::mem::size_of::<u8>());
}

#[test]
fn test_approximate_size_single_byte() {
    let pattern = FreqyPacked {
        pat: vec![10],
        char_len: 1,
        rare1: 10,
        rare1i: 0,
        rare2: 10,
        rare2i: 0,
    };
    assert_eq!(pattern.approximate_size(), 1 * std::mem::size_of::<u8>());
}

