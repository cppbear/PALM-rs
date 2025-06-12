// Answer 0

#[test]
fn test_new_with_non_empty_pattern() {
    let pat = vec![1, 2, 3, 4, 2, 1]; 
    let freqy_packed = FreqyPacked::new(pat.clone());
    assert_eq!(freqy_packed.pat, pat);
    assert_eq!(freqy_packed.char_len, 6);
    assert_eq!(freqy_packed.rare1, 1);
    assert_eq!(freqy_packed.rare1i, 5);
    assert_eq!(freqy_packed.rare2, 2);
    assert_eq!(freqy_packed.rare2i, 4);
}

#[test]
fn test_new_with_identical_elements() {
    let pat = vec![2, 2, 2, 2]; 
    let freqy_packed = FreqyPacked::new(pat.clone());
    assert_eq!(freqy_packed.pat, pat);
    assert_eq!(freqy_packed.char_len, 4);
    assert_eq!(freqy_packed.rare1, 2);
    assert_eq!(freqy_packed.rare1i, 3);
    assert_eq!(freqy_packed.rare2, 2);
    assert_eq!(freqy_packed.rare2i, 3);
}

#[test]
fn test_new_with_two_distinct_elements() {
    let pat = vec![4, 5]; 
    let freqy_packed = FreqyPacked::new(pat.clone());
    assert_eq!(freqy_packed.pat, pat);
    assert_eq!(freqy_packed.char_len, 2);
    assert_eq!(freqy_packed.rare1, 4);
    assert_eq!(freqy_packed.rare1i, 0);
    assert_eq!(freqy_packed.rare2, 5);
    assert_eq!(freqy_packed.rare2i, 1);
}

#[test]
fn test_new_with_empty_vector() {
    let pat: Vec<u8> = vec![];
    let freqy_packed = FreqyPacked::new(pat);
    assert_eq!(freqy_packed.pat.len(), 0);
    assert_eq!(freqy_packed.char_len, 0);
    assert_eq!(freqy_packed.rare1, 0);
    assert_eq!(freqy_packed.rare1i, 0);
    assert_eq!(freqy_packed.rare2, 0);
    assert_eq!(freqy_packed.rare2i, 0);
}

