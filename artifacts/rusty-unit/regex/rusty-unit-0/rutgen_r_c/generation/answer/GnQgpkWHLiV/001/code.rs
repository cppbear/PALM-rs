// Answer 0

#[test]
fn test_new_boyer_moore_search_single_character_pattern() {
    let pattern = vec![b'a'];
    let searcher = BoyerMooreSearch::new(pattern.clone());
    
    assert_eq!(searcher.pattern, pattern);
    assert_eq!(searcher.skip_table, vec![0; 256]);
    assert_eq!(searcher.guard, b'a');
    assert_eq!(searcher.guard_reverse_idx, 0);
    assert_eq!(searcher.md2_shift, 0xDEADBEAF);
}

#[test]
fn test_new_boyer_moore_search_repeated_pattern() {
    let pattern = b"abcabc".to_vec();
    let searcher = BoyerMooreSearch::new(pattern.clone());
    
    assert_eq!(searcher.pattern, pattern);
    assert_eq!(searcher.skip_table, vec![4, 3, 2, 5, 4, 1]);
    assert_eq!(searcher.guard, b'c');
    assert_eq!(searcher.guard_reverse_idx, 5);
    assert_eq!(searcher.md2_shift, 3);
}

#[test]
fn test_new_boyer_moore_search_mixed_characters() {
    let pattern = b"hello, world!".to_vec();
    let searcher = BoyerMooreSearch::new(pattern.clone());
    
    assert_eq!(searcher.pattern, pattern);
    assert_eq!(searcher.skip_table, vec![12, 13, 11, 12, 9, 12, 12, 13, 12, 12, 12, 12, 12, 12, 12, 12]);
    assert_eq!(searcher.guard, b'!');
    assert_eq!(searcher.guard_reverse_idx, 12);
    assert_eq!(searcher.md2_shift, 1);
}

#[test]
#[should_panic]
fn test_new_boyer_moore_search_empty_pattern() {
    let pattern: Vec<u8> = vec![];
    BoyerMooreSearch::new(pattern);
}

