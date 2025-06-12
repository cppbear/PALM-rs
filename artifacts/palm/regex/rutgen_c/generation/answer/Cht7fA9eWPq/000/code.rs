// Answer 0

#[test]
fn test_literal_searcher_prefixes_empty() {
    struct EmptyLiterals;

    impl Literals for EmptyLiterals {
        fn literals(&self) -> &[&[u8]] {
            &[]
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

    let lits = EmptyLiterals;
    let searcher = LiteralSearcher::prefixes(lits);
    assert!(searcher.is_empty());
}

#[test]
fn test_literal_searcher_prefixes_single() {
    struct SingleLiterals;

    impl Literals for SingleLiterals {
        fn literals(&self) -> &[&[u8]] {
            &[b"prefix"]
        }
        fn all_complete(&self) -> bool {
            true
        }
        fn longest_common_prefix(&self) -> &[u8] {
            b"pre"
        }
        fn longest_common_suffix(&self) -> &[u8] {
            &[]
        }
    }

    let lits = SingleLiterals;
    let searcher = LiteralSearcher::prefixes(lits);
    assert_eq!(searcher.lcp().pat, b"pre".to_vec());
}

#[test]
fn test_literal_searcher_prefixes_multiple() {
    struct MultipleLiterals;

    impl Literals for MultipleLiterals {
        fn literals(&self) -> &[&[u8]] {
            &[b"prefix", b"prefixes", b"preliminary"]
        }
        fn all_complete(&self) -> bool {
            false
        }
        fn longest_common_prefix(&self) -> &[u8] {
            b"pre"
        }
        fn longest_common_suffix(&self) -> &[u8] {
            &[]
        }
    }

    let lits = MultipleLiterals;
    let searcher = LiteralSearcher::prefixes(lits);
    assert_eq!(searcher.lcp().pat, b"pre".to_vec());
    assert!(!searcher.is_empty());
}

