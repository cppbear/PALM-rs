// Answer 0

#[test]
fn test_find_empty_haystack_with_empty_matcher() {
    let searcher = LiteralSearcher::empty();
    let haystack: &[u8] = &[];
    let _ = searcher.find(haystack);
}

