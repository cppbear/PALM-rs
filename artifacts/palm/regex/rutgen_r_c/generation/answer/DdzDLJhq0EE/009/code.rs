// Answer 0

#[test]
fn test_new_non_empty_pattern_with_distinct_rarest_bytes() {
    let pat = vec![5, 3, 5, 2, 3];
    let packed = FreqyPacked::new(pat.clone());
    
    assert_eq!(packed.pat, pat);
    assert_eq!(packed.char_len, 5);
    assert_eq!(packed.rare1, 2);
    assert_eq!(packed.rare1i, 3);
    assert_eq!(packed.rare2, 3);
    assert_eq!(packed.rare2i, 1);
}

#[test]
fn test_new_non_empty_pattern_with_identical_rarest_bytes() {
    let pat = vec![4, 4, 3, 4, 3];
    let packed = FreqyPacked::new(pat.clone());
    
    assert_eq!(packed.pat, pat);
    assert_eq!(packed.char_len, 5);
    assert_eq!(packed.rare1, 3);
    assert_eq!(packed.rare1i, 4);
    assert_eq!(packed.rare2, 4);
    assert_eq!(packed.rare2i, 0);
}

#[test]
fn test_new_single_element_pattern() {
    let pat = vec![7];
    let packed = FreqyPacked::new(pat.clone());
    
    assert_eq!(packed.pat, pat);
    assert_eq!(packed.char_len, 1);
    assert_eq!(packed.rare1, 7);
    assert_eq!(packed.rare1i, 0);
    assert_eq!(packed.rare2, 7);
    assert_eq!(packed.rare2i, 0);
}

#[test]
fn test_new_pattern_with_repeated_bytes() {
    let pat = vec![8, 8, 8, 9, 10];
    let packed = FreqyPacked::new(pat.clone());
    
    assert_eq!(packed.pat, pat);
    assert_eq!(packed.char_len, 5);
    assert_eq!(packed.rare1, 9);
    assert_eq!(packed.rare1i, 3);
    assert_eq!(packed.rare2, 8);
    assert_eq!(packed.rare2i, 0);
}

#[test]
fn test_new_empty_pattern_returns_empty_freqy_packed() {
    let pat: Vec<u8> = vec![];
    let packed = FreqyPacked::new(pat.clone());
    
    assert_eq!(packed.pat, vec![]);
    assert_eq!(packed.char_len, 0);
    assert_eq!(packed.rare1, 0);
    assert_eq!(packed.rare1i, 0);
    assert_eq!(packed.rare2, 0);
    assert_eq!(packed.rare2i, 0);
}

