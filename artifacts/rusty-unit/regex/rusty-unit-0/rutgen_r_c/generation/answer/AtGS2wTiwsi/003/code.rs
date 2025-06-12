// Answer 0

#[test]
fn test_find_start_no_match() {
    struct DummyLiterals;
    impl DummyLiterals {
        fn empty() -> Literals {
            Literals::empty()
        }
        fn longest_common_prefix(&self) -> &[u8] {
            &[]
        }
        fn longest_common_suffix(&self) -> &[u8] {
            &[]
        }
        fn all_complete(&self) -> bool {
            true
        }
    }

    let searcher = LiteralSearcher::empty();
    let haystack = b"abc"; // haystack of length 3
    let result = searcher.find_start(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_start_exact_length_no_match() {
    struct DummyLiterals;
    impl DummyLiterals {
        fn empty() -> Literals {
            Literals::empty()
        }
        fn longest_common_prefix(&self) -> &[u8] {
            b"xyz" // pattern length 3, does not match haystack
        }
        fn longest_common_suffix(&self) -> &[u8] {
            &[]
        }
        fn all_complete(&self) -> bool {
            true
        }
    }

    let searcher = LiteralSearcher::prefixes(DummyLiterals::empty());
    let haystack = b"abc"; // haystack of length 3
    let result = searcher.find_start(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_start_empty_iterator() {
    struct DummyLiterals;
    impl DummyLiterals {
        fn empty() -> Literals {
            Literals::empty()
        }
        fn longest_common_prefix(&self) -> &[u8] {
            &[]
        }
        fn longest_common_suffix(&self) -> &[u8] {
            &[]
        }
        fn all_complete(&self) -> bool {
            true
        }
    }

    let searcher = LiteralSearcher::empty();
    let haystack = b""; // empty haystack
    let result = searcher.find_start(haystack);
    assert_eq!(result, None);
}

