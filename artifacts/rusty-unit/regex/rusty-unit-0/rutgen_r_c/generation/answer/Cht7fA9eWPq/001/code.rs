// Answer 0

#[test]
fn test_prefixes_empty_literals() {
    let lits = Literals::empty();
    let searcher = LiteralSearcher::prefixes(lits);
    assert_eq!(searcher.is_empty(), true);
}

#[test]
fn test_prefixes_single_literal() {
    let literals = vec![b"a".to_vec()];
    let lits = Literals::from_vec(literals);
    let searcher = LiteralSearcher::prefixes(lits);
    assert!(!searcher.is_empty());
    assert_eq!(searcher.len(), 1);
}

#[test]
fn test_prefixes_multiple_literals() {
    let literals = vec![b"apple".to_vec(), b"apricot".to_vec()];
    let lits = Literals::from_vec(literals);
    let searcher = LiteralSearcher::prefixes(lits);
    assert!(!searcher.is_empty());
    assert_eq!(searcher.len(), 2);
}

#[test]
fn test_prefixes_with_long_common_prefix() {
    let literals = vec![b"dog".to_vec(), b"doghouse".to_vec(), b"dogma".to_vec()];
    let lits = Literals::from_vec(literals);
    let searcher = LiteralSearcher::prefixes(lits);
    assert!(!searcher.is_empty());
    assert_eq!(searcher.lcp().pat, b"dog");
}

#[test]
fn test_prefixes_multiple_short_literals() {
    let literals = vec![b"zebra".to_vec(), b"zipping".to_vec(), b"zephyrous".to_vec()];
    let lits = Literals::from_vec(literals);
    let searcher = LiteralSearcher::prefixes(lits);
    assert!(!searcher.is_empty());
    assert_eq!(searcher.len(), 3);
}

