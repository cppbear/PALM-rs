// Answer 0

#[test]
fn test_find_empty_matcher() {
    let searcher = LiteralSearcher::empty();
    let haystack = b"test string";
    assert_eq!(searcher.find(haystack), Some((0, 0)));
}

#[test]
fn test_find_single_byte_set() {
    let sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b't'], // match a single byte
        complete: false,
        all_ascii: true,
    };
    let matcher = Matcher::Bytes(sset);
    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher,
    };
    let haystack = b"test string";
    assert_eq!(searcher.find(haystack), Some((0, 1))); // 't' is found at index 0
}

#[test]
fn test_find_freqy_packed() {
    let freqy = FreqyPacked::new(b"test".to_vec());
    let matcher = Matcher::FreqyPacked(freqy);
    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher,
    };
    let haystack = b"this is a test string";
    assert_eq!(searcher.find(haystack), Some((10, 14))); // "test" is found at index 10
}

#[test]
fn test_find_boyer_moore() {
    let pattern = b"pattern".to_vec();
    let boyer_moore = BoyerMooreSearch::new(pattern.clone());
    let matcher = Matcher::BoyerMoore(boyer_moore);
    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher,
    };
    let haystack = b"this is a pattern matching test";
    assert_eq!(searcher.find(haystack), Some((10, 17))); // "pattern" is found at index 10
}

