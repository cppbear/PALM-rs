// Answer 0

#[test]
fn test_literal_searcher_empty() {
    let searcher = LiteralSearcher::empty();
    assert!(searcher.complete() == false);
    assert!(searcher.is_empty() == true);
    assert!(searcher.len() == 0);
    let haystack: &[u8] = b"example string";
    assert!(searcher.find(haystack).is_none());
    assert!(searcher.find_start(haystack).is_none());
    assert!(searcher.find_end(haystack).is_none());
    
    let lcp = searcher.lcp();
    let lcs = searcher.lcs();
    assert!(lcp.pat.is_empty());
    assert!(lcs.pat.is_empty());
}

