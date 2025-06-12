// Answer 0

#[test]
fn test_suffixes_empty_literals() {
    let lits = Literals::empty();
    let searcher = LiteralSearcher::suffixes(lits);
    assert!(searcher.is_empty());
}

#[test]
fn test_suffixes_single_literal() {
    let lits = Literals::from(vec![b"test".to_vec()]);
    let searcher = LiteralSearcher::suffixes(lits);
    assert_eq!(searcher.len(), 1);
    assert!(!searcher.is_empty());
}

#[test]
fn test_suffixes_multiple_literals() {
    let lits = Literals::from(vec![b"abc".to_vec(), b"def".to_vec()]);
    let searcher = LiteralSearcher::suffixes(lits);
    assert!(searcher.complete());
    assert_eq!(searcher.len(), 2);
}

#[test]
fn test_suffixes_large_literals() {
    let literals = (0..32).map(|i| format!("literal{}", i).into_bytes()).collect::<Vec<_>>();
    let lits = Literals::from(literals);
    let searcher = LiteralSearcher::suffixes(lits);
    assert!(!searcher.is_empty());
    assert_eq!(searcher.len(), 32);
}

#[test]
fn test_suffixes_ascii_literals() {
    let lits = Literals::from(vec![b"foo".to_vec(), b"bar".to_vec()]);
    let searcher = LiteralSearcher::suffixes(lits);
    assert!(searcher.complete());
}

