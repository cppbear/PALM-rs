// Answer 0

#[test]
fn test_new_boyer_moore_search() {
    let pattern = b"abc".to_vec();
    let searcher = BoyerMooreSearch::new(pattern.clone());

    assert_eq!(searcher.pattern, pattern);
    assert_eq!(searcher.skip_table, vec![2, 1, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                                         3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
                                         3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
                                         3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);
    assert_eq!(searcher.guard, b'a');
    assert_eq!(searcher.guard_reverse_idx, 2);
    assert_eq!(searcher.md2_shift, 1);
}

#[test]
fn test_new_boyer_moore_search_empty_pattern() {
    let pattern = b"".to_vec();
    let result = std::panic::catch_unwind(|| {
        BoyerMooreSearch::new(pattern);
    });
    assert!(result.is_err());
}

