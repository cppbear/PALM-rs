// Answer 0

#[test]
fn test_lcs_non_empty() {
    #[derive(Clone, Debug)]
    struct DummyLiteral {
        patterns: Vec<u8>,
    }

    #[derive(Clone, Debug)]
    struct DummyMatcher;

    impl DummyMatcher {
        fn new() -> Self {
            DummyMatcher
        }
    }

    let freqy_packed = FreqyPacked {
        pat: vec![b'a', b'b', b'c'],
        char_len: 3,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };

    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]), // empty LCP for this test
        lcs: freqy_packed,
        matcher: Matcher::Empty,
    };

    let lcs_result = searcher.lcs();
    assert_eq!(lcs_result.pat, vec![b'a', b'b', b'c']);
    assert_eq!(lcs_result.char_len, 3);
}

#[test]
fn test_lcs_empty() {
    #[derive(Clone, Debug)]
    struct DummyLiteral {
        patterns: Vec<u8>,
    }

    #[derive(Clone, Debug)]
    struct DummyMatcher;

    impl DummyMatcher {
        fn new() -> Self {
            DummyMatcher
        }
    }

    let freqy_packed = FreqyPacked {
        pat: vec![],
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };

    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]), // empty LCP for this test
        lcs: freqy_packed,
        matcher: Matcher::Empty,
    };

    let lcs_result = searcher.lcs();
    assert_eq!(lcs_result.pat, vec![]);
    assert_eq!(lcs_result.char_len, 0);
}

