// Answer 0

#[test]
fn test_empty_freqy_packed() {
    let packed = FreqyPacked::empty();
    assert_eq!(packed.pat, vec![]);
    assert_eq!(packed.char_len, 0);
    assert_eq!(packed.rare1, 0);
    assert_eq!(packed.rare1i, 0);
    assert_eq!(packed.rare2, 0);
    assert_eq!(packed.rare2i, 0);
}

