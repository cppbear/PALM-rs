// Answer 0

#[test]
fn test_lcp_empty() {
    let searcher = LiteralSearcher::empty();
    assert_eq!(searcher.lcp().pat.len(), 0);
    assert_eq!(searcher.lcp().char_len, 0);
}

#[test]
fn test_lcp_with_prefix() {
    let prefix_lits = Literals::from(vec![b"test", b"testing"]);
    let searcher = LiteralSearcher::prefixes(prefix_lits);
    assert!(searcher.lcp().pat.len() > 0);
}

#[test]
fn test_lcp_with_suffix() {
    let suffix_lits = Literals::from(vec![b"hello", b"lo"]);
    let searcher = LiteralSearcher::suffixes(suffix_lits);
    assert!(searcher.lcp().pat.len() > 0);
}

#[test]
fn test_lcp_another_prefix() {
    let prefix_lits = Literals::from(vec![b"example", b"exceed"]);
    let searcher = LiteralSearcher::prefixes(prefix_lits);
    assert!(searcher.lcp().pat.len() > 0);
    assert_eq!(searcher.lcp().char_len, 2);
}

