// Answer 0

#[test]
fn test_literal_searcher_new_with_empty_literals() {
    struct MockLiterals;

    impl Literals for MockLiterals {
        fn empty() -> Self {
            MockLiterals
        }
        
        fn all_complete(&self) -> bool {
            true
        }
        
        fn longest_common_prefix(&self) -> &[u8] {
            &[]
        }
        
        fn longest_common_suffix(&self) -> &[u8] {
            &[]
        }
    }

    let lits = MockLiterals::empty();
    let matcher = Matcher::Empty;

    let searcher = LiteralSearcher::new(lits, matcher);
    assert_eq!(searcher.complete, true);
    assert_eq!(searcher.lcp.len(), 0);
    assert_eq!(searcher.lcs.len(), 0);
}

#[test]
fn test_literal_searcher_new_with_non_empty_literals() {
    struct MockLiterals;

    impl Literals for MockLiterals {
        fn empty() -> Self {
            MockLiterals
        }

        fn all_complete(&self) -> bool {
            false
        }

        fn longest_common_prefix(&self) -> &[u8] {
            b"abc"
        }

        fn longest_common_suffix(&self) -> &[u8] {
            b"xyz"
        }
    }

    let lits = MockLiterals::empty();
    let matcher = Matcher::Empty;

    let searcher = LiteralSearcher::new(lits, matcher);
    assert_eq!(searcher.complete, false);
    assert_eq!(searcher.lcp.pat, b"abc");
    assert_eq!(searcher.lcs.pat, b"xyz");
}

