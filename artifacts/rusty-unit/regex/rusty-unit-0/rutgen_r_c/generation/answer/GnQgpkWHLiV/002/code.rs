// Answer 0

#[test]
#[should_panic]
fn test_boyer_moore_search_new_empty_pattern() {
    let pattern: Vec<u8> = vec![];
    let _searcher = BoyerMooreSearch::new(pattern);
}

#[test]
fn test_boyer_moore_search_new_single_character_pattern() {
    let pattern = vec![b'a'];
    let searcher = BoyerMooreSearch::new(pattern);
    assert_eq!(searcher.pattern, vec![b'a']);
    assert_eq!(searcher.skip_table.len(), 256);
    assert_eq!(searcher.skip_table[b'a' as usize], 0);
    assert_eq!(searcher.guard, b'a');
    assert_eq!(searcher.guard_reverse_idx, 0);
    assert_eq!(searcher.md2_shift, 0xDEADBEAF);
}

#[test]
fn test_boyer_moore_search_new_multiple_character_pattern() {
    let pattern = vec![b'a', b'b', b'a'];
    let searcher = BoyerMooreSearch::new(pattern);
    assert_eq!(searcher.pattern, vec![b'a', b'b', b'a']);
    assert_eq!(searcher.skip_table.len(), 256);
    assert_eq!(searcher.skip_table[b'a' as usize], 1);
    assert_eq!(searcher.skip_table[b'b' as usize], 2);
    assert_eq!(searcher.guard, b'b');
    assert_eq!(searcher.guard_reverse_idx, 1);
    assert_eq!(searcher.md2_shift, 1);
}

