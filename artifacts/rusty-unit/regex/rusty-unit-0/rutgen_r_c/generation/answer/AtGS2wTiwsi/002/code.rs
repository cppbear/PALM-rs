// Answer 0

#[test]
fn test_find_start_success() {
    #[derive(Clone, Debug)]
    struct DummyMatcher {
        pattern: Vec<u8>,
    }

    #[derive(Clone, Debug)]
    struct DummyLiterals {
        patterns: Vec<Vec<u8>>,
    }

    impl DummyMatcher {
        fn new(pattern: Vec<u8>) -> Self {
            Self { pattern }
        }
    }

    impl DummyLiterals {
        fn longest_common_prefix(&self) -> &[u8] {
            &self.patterns[0]
        }

        fn longest_common_suffix(&self) -> &[u8] {
            &self.patterns[0]
        }

        fn all_complete(&self) -> bool {
            true
        }
    }

    let literals = DummyLiterals { patterns: vec![b"test".to_vec()] };
    let matcher = DummyMatcher::new(b"test".to_vec());

    let searcher = LiteralSearcher::new(literals, Matcher::FreqyPacked(FreqyPacked {
        pat: b"test".to_vec(),
        char_len: 4,
        rare1: b't',
        rare1i: 0,
        rare2: b'e',
        rare2i: 1,
    }));

    let result = searcher.find_start(b"test");
    assert_eq!(result, Some((0, 4)));
}

