// Answer 0

#[test]
fn test_approximate_size_with_freqy_packed() {
    struct DummyLiterals;

    impl DummyLiterals {
        fn empty() -> Self {
            DummyLiterals
        }

        fn longest_common_prefix(&self) -> Vec<u8> {
            vec![0]
        }

        fn longest_common_suffix(&self) -> Vec<u8> {
            vec![1]
        }

        fn all_complete(&self) -> bool {
            true
        }
    }

    let literals = DummyLiterals::empty();
    let matcher = Matcher::FreqyPacked(FreqyPacked {
        pat: vec![0, 1, 2],
        char_len: 3,
        rare1: 0,
        rare1i: 0,
        rare2: 1,
        rare2i: 1,
    });

    let searcher = LiteralSearcher::new(literals, matcher);
    assert_eq!(searcher.approximate_size(), 3 * std::mem::size_of::<u8>());
}

#[test]
fn test_approximate_size_with_empty_freqy_packed() {
    struct DummyLiterals;

    impl DummyLiterals {
        fn empty() -> Self {
            DummyLiterals
        }

        fn longest_common_prefix(&self) -> Vec<u8> {
            vec![0]
        }

        fn longest_common_suffix(&self) -> Vec<u8> {
            vec![1]
        }

        fn all_complete(&self) -> bool {
            true
        }
    }

    let literals = DummyLiterals::empty();
    let matcher = Matcher::FreqyPacked(FreqyPacked {
        pat: Vec::new(),
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    });

    let searcher = LiteralSearcher::new(literals, matcher);
    assert_eq!(searcher.approximate_size(), 0);
}

