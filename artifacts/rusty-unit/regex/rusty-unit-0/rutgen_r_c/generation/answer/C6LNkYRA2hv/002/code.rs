// Answer 0

#[test]
fn test_is_suffix_equal() {
    let pattern = vec![1, 2, 3];
    let freqy_packed = FreqyPacked::new(pattern.clone());
    let text = vec![1, 2, 3];
    assert!(freqy_packed.is_suffix(&text));
}

#[test]
fn test_is_suffix_not_equal() {
    let pattern = vec![1, 2, 3];
    let freqy_packed = FreqyPacked::new(pattern.clone());
    let text = vec![4, 5, 6];
    assert!(!freqy_packed.is_suffix(&text));
}

#[test]
fn test_is_suffix_empty_text() {
    let pattern = vec![1, 2, 3];
    let freqy_packed = FreqyPacked::new(pattern);
    let text = vec![1, 2, 3];
    assert!(freqy_packed.is_suffix(&text));
}

#[test]
fn test_is_suffix_empty_pattern() {
    let pattern = vec![];
    let freqy_packed = FreqyPacked::new(pattern);
    let text = vec![];
    assert!(freqy_packed.is_suffix(&text));
}

#[test]
fn test_is_suffix_longer_text() {
    let pattern = vec![7, 8, 9];
    let freqy_packed = FreqyPacked::new(pattern.clone());
    let text = vec![1, 2, 3, 7, 8, 9];
    assert!(freqy_packed.is_suffix(&text));
}

#[test]
fn test_is_suffix_partial_match() {
    let pattern = vec![1, 2, 3];
    let freqy_packed = FreqyPacked::new(pattern);
    let text = vec![1, 2];
    assert!(!freqy_packed.is_suffix(&text));
}

