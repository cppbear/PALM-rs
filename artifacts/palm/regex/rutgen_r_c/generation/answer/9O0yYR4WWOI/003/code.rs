// Answer 0

#[test]
fn test_len_with_ac_matcher() {
    #[derive(Debug)]
    struct MockAc {
        count: usize,
    }

    impl MockAc {
        fn len(&self) -> usize {
            self.count
        }
    }

    #[derive(Debug)]
    struct MockLiterals;

    impl MockLiterals {
        fn new() -> Self {
            MockLiterals
        }
        
        fn all_complete(&self) -> bool {
            true
        }

        fn longest_common_prefix(&self) -> &[u8] {
            b""
        }

        fn longest_common_suffix(&self) -> &[u8] {
            b""
        }
        
        fn patterns(&self) -> Vec<MockAc> {
            vec![MockAc { count: 5 }]
        }
    }

    let literals = MockLiterals::new();
    let matcher = Matcher::AC(MockAc { count: 5 });
    let searcher = LiteralSearcher::new(literals, matcher);

    assert_eq!(searcher.len(), 5);
}

#[test]
fn test_len_with_empty_matcher() {
    let literals = MockLiterals::new();
    let matcher = Matcher::Empty;
    let searcher = LiteralSearcher::new(literals, matcher);

    assert_eq!(searcher.len(), 0);
}

#[test]
fn test_len_with_bytes_matcher() {
    let literals = MockLiterals::new();
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: b"abc".to_vec(),
        complete: true,
        all_ascii: true,
    };
    let matcher = Matcher::Bytes(single_byte_set);
    let searcher = LiteralSearcher::new(literals, matcher);

    assert_eq!(searcher.len(), 3);
}

#[test]
fn test_len_with_freqy_packed_matcher() {
    let literals = MockLiterals::new();
    let freqy_packed = FreqyPacked {
        pat: b"pattern".to_vec(),
        char_len: 7,
        rare1: b'p',
        rare1i: 0,
        rare2: b'a',
        rare2i: 1,
    };
    let matcher = Matcher::FreqyPacked(freqy_packed);
    let searcher = LiteralSearcher::new(literals, matcher);

    assert_eq!(searcher.len(), 1);
}

#[test]
fn test_len_with_boyer_moore_matcher() {
    let literals = MockLiterals::new();
    let matcher = Matcher::BoyerMoore(MockAc { count: 1 });
    let searcher = LiteralSearcher::new(literals, matcher);

    assert_eq!(searcher.len(), 1);
}

