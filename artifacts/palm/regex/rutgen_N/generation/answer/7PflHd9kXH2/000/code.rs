// Answer 0

#[test]
fn test_empty() {
    struct FreqyPacked {
        pat: Vec<u8>,
        char_len: usize,
        rare1: usize,
        rare1i: usize,
        rare2: usize,
        rare2i: usize,
    }

    fn empty() -> FreqyPacked {
        FreqyPacked {
            pat: vec![],
            char_len: 0,
            rare1: 0,
            rare1i: 0,
            rare2: 0,
            rare2i: 0,
        }
    }

    let result = empty();
    assert_eq!(result.pat.len(), 0);
    assert_eq!(result.char_len, 0);
    assert_eq!(result.rare1, 0);
    assert_eq!(result.rare1i, 0);
    assert_eq!(result.rare2, 0);
    assert_eq!(result.rare2i, 0);
}

