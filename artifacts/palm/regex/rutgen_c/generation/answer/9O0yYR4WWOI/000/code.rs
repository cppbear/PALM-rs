// Answer 0

#[test]
fn test_literal_searcher_len_empty() {
    let searcher = LiteralSearcher::empty();
    assert_eq!(searcher.len(), 0);
}

#[test]
fn test_literal_searcher_len_bytes() {
    let single_byte_set = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a', b'b', b'c'],
        complete: false,
        all_ascii: true,
    };
    let matcher = Matcher::Bytes(single_byte_set);
    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(vec![]), // Contains no prefixes
        lcs: FreqyPacked::new(vec![]), // Contains no suffixes
        matcher,
    };
    assert_eq!(searcher.len(), 3);
}

#[test]
fn test_literal_searcher_len_freqy_packed() {
    let freqy_packed = FreqyPacked {
        pat: b"test".to_vec(),
        char_len: 4,
        rare1: b't',
        rare1i: 0,
        rare2: b'e',
        rare2i: 1,
    };
    let matcher = Matcher::FreqyPacked(freqy_packed);
    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(vec![]), // Contains no prefixes
        lcs: FreqyPacked::new(vec![]), // Contains no suffixes
        matcher,
    };
    assert_eq!(searcher.len(), 1);
}

#[test]
fn test_literal_searcher_len_boyer_moore() {
    let boyer_moore_search = BoyerMooreSearch {
        pattern: b"sample".to_vec(),
        // Initialize other necessary fields if needed
    };
    let matcher = Matcher::BoyerMoore(boyer_moore_search);
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]), // Contains no prefixes
        lcs: FreqyPacked::new(vec![]), // Contains no suffixes
        matcher,
    };
    assert_eq!(searcher.len(), 1);
}

#[test]
fn test_literal_searcher_len_aho_corasick() {
    let aut = FullAcAutomaton::new(vec![Literal { pattern: b"test".to_vec() }]);
    let matcher = Matcher::AC(aut);
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]), // Contains no prefixes
        lcs: FreqyPacked::new(vec![]), // Contains no suffixes
        matcher,
    };
    assert_eq!(searcher.len(), 1);
}

