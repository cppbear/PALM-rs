// Answer 0

#[test]
fn test_new_empty_pattern() {
    let pat: Vec<u8> = vec![];
    let result = FreqyPacked::new(pat);
    assert_eq!(result.len(), 0);
    assert_eq!(result.char_len(), 0);
    assert!(result.pat.is_empty());
    assert_eq!(result.rare1, 0);
    assert_eq!(result.rare1i, 0);
    assert_eq!(result.rare2, 0);
    assert_eq!(result.rare2i, 0);
}

