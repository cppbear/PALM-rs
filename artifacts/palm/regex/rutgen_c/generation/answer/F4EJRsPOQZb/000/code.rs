// Answer 0

#[test]
fn test_suffixes_empty() {
    struct DummyLiterals;
    impl Literals for DummyLiterals {
        fn literals(&self) -> Vec<&[u8]> { vec![] }
        fn longest_common_suffix(&self) -> &[u8] { &[] }
        fn all_complete(&self) -> bool { true }
    }

    let lits = DummyLiterals;
    let searcher = LiteralSearcher::suffixes(lits);
    assert!(searcher.is_empty());
    assert_eq!(searcher.len(), 0);
}

#[test]
fn test_suffixes_single_literal() {
    struct SingleLiteral;
    impl Literals for SingleLiteral {
        fn literals(&self) -> Vec<&[u8]> { vec![b"test"] }
        fn longest_common_suffix(&self) -> &[u8] { b"test" }
        fn all_complete(&self) -> bool { true }
    }

    let lits = SingleLiteral;
    let searcher = LiteralSearcher::suffixes(lits);
    assert!(!searcher.is_empty());
    assert_eq!(searcher.len(), 4);
}

#[test]
fn test_suffixes_multiple_literals() {
    struct MultiLiteral;
    impl Literals for MultiLiteral {
        fn literals(&self) -> Vec<&[u8]> { vec![b"test", b"best"] }
        fn longest_common_suffix(&self) -> &[u8] { b"st" }
        fn all_complete(&self) -> bool { true }
    }

    let lits = MultiLiteral;
    let searcher = LiteralSearcher::suffixes(lits);
    assert!(!searcher.is_empty());
    assert_eq!(searcher.len(), 2);
}

