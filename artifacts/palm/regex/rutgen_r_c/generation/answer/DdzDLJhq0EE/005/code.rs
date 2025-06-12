// Answer 0

#[test]
fn test_new_freqy_packed_with_non_empty_pattern() {
    let pattern = vec![3, 1, 4, 1, 5]; // valid non-empty pattern
    let packed = FreqyPacked::new(pattern.clone());
    assert_eq!(packed.pat, pattern);
    assert_eq!(packed.char_len, char_len_lossy(&pattern));
    assert_eq!(packed.rare1, 1); // rarest byte
    assert_eq!(packed.rare1i, 1); // index of rarest byte
    assert_eq!(packed.rare2, 3); // second rarest byte
    assert_eq!(packed.rare2i, 0); // index of second rarest byte
}

#[test]
fn test_new_freqy_packed_with_distinct_rare_bytes() {
    let pattern = vec![5, 9, 2, 7, 3]; // non-empty with distinct rarities
    let packed = FreqyPacked::new(pattern.clone());
    assert_eq!(packed.pat, pattern);
    assert_eq!(packed.char_len, char_len_lossy(&pattern));
    assert_eq!(packed.rare1, 2); // rarest byte
    assert_eq!(packed.rare1i, 2); // index of rarest byte
    assert_eq!(packed.rare2, 3); // second rarest byte
    assert_eq!(packed.rare2i, 4); // index of second rarest byte
}

#[test]
fn test_new_freqy_packed_with_repeated_bytes() {
    let pattern = vec![7, 7, 8, 8, 9]; // includes repetitions
    let packed = FreqyPacked::new(pattern.clone());
    assert_eq!(packed.pat, pattern);
    assert_eq!(packed.char_len, char_len_lossy(&pattern));
    assert_eq!(packed.rare1, 7); // rarest byte
    assert_eq!(packed.rare1i, 1); // last occurrence of rare1
    assert_eq!(packed.rare2, 8); // second rarest byte
    assert_eq!(packed.rare2i, 3); // last occurrence of rare2
}

#[test]
fn test_new_freqy_packed_with_single_character() {
    let pattern = vec![0]; // single character edge case
    let packed = FreqyPacked::new(pattern.clone());
    assert_eq!(packed.pat, pattern);
    assert_eq!(packed.char_len, char_len_lossy(&pattern));
    assert_eq!(packed.rare1, 0); // only byte present
    assert_eq!(packed.rare1i, 0); // only index available
    assert_eq!(packed.rare2, 0); // rare1 and rare2 are the same
    assert_eq!(packed.rare2i, 0); // only index available
}

#[test]
fn test_new_freqy_packed_with_empty_pattern() {
    let pattern: Vec<u8> = vec![];
    let packed = FreqyPacked::new(pattern.clone());
    assert_eq!(packed.pat, vec![]);
    assert_eq!(packed.char_len, 0);
    assert_eq!(packed.rare1, 0);
    assert_eq!(packed.rare1i, 0);
    assert_eq!(packed.rare2, 0);
    assert_eq!(packed.rare2i, 0);
}

