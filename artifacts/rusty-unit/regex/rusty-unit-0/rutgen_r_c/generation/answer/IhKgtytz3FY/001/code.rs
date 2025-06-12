// Answer 0

#[test]
fn test_literal_searcher_new_empty_literals() {
    struct DummyLiterals;
    impl Literals for DummyLiterals {
        fn empty() -> Self { DummyLiterals }
        fn all_complete(&self) -> bool { true }
        fn longest_common_prefix(&self) -> &[u8] { &[] }
        fn longest_common_suffix(&self) -> &[u8] { &[] }
    }

    let lits = DummyLiterals::empty();
    let matcher = Matcher::Empty;

    let searcher = LiteralSearcher::new(lits, matcher);

    assert!(searcher.complete);
    assert_eq!(searcher.lcp.len(), 0);
    assert_eq!(searcher.lcs.len(), 0);
}

#[test]
fn test_literal_searcher_new_non_empty_literals() {
    struct TestLiterals;
    impl Literals for TestLiterals {
        fn empty() -> Self { TestLiterals }
        fn all_complete(&self) -> bool { true }
        fn longest_common_prefix(&self) -> &[u8] { b"abc" }
        fn longest_common_suffix(&self) -> &[u8] { b"xyz" }
    }

    let lits = TestLiterals::empty();
    let matcher = Matcher::Bytes(SingleByteSet::new());

    let searcher = LiteralSearcher::new(lits, matcher);

    assert!(searcher.complete);
    assert_eq!(searcher.lcp.pat, vec![97, 98, 99]); // ASCII for 'abc'
    assert_eq!(searcher.lcs.pat, vec![120, 121, 122]); // ASCII for 'xyz'
}

#[test]
fn test_literal_searcher_new_incomplete_literals() {
    struct IncompleteLiterals;
    impl Literals for IncompleteLiterals {
        fn empty() -> Self { IncompleteLiterals }
        fn all_complete(&self) -> bool { false }
        fn longest_common_prefix(&self) -> &[u8] { b"abc" }
        fn longest_common_suffix(&self) -> &[u8] { b"xyz" }
    }

    let lits = IncompleteLiterals::empty();
    let matcher = Matcher::FreqyPacked(FreqyPacked::new(vec![97, 98, 99])); // 'abc'

    let searcher = LiteralSearcher::new(lits, matcher);

    assert!(!searcher.complete);
    assert_eq!(searcher.lcp.pat, vec![97, 98, 99]); // ASCII for 'abc'
    assert_eq!(searcher.lcs.pat, vec![120, 121, 122]); // ASCII for 'xyz'
}

