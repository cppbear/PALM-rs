// Answer 0

#[test]
fn test_literal_searcher_empty() {
    struct TestLiterals;

    impl TestLiterals {
        fn empty() -> Self {
            TestLiterals
        }

        fn all_complete(&self) -> bool {
            true
        }

        fn longest_common_prefix(&self) -> Vec<u8> {
            vec![]
        }

        fn longest_common_suffix(&self) -> Vec<u8> {
            vec![]
        }
    }

    let searcher = LiteralSearcher::empty();
    assert_eq!(searcher.complete, true);
    assert!(searcher.is_empty());
    assert_eq!(searcher.len(), 0);
}

