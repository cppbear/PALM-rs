// Answer 0

#[test]
fn test_find_bytes_match_empty_haystack() {
    let sset = SingleByteSet {
        sparse: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        dense: vec![],
        complete: false,
        all_ascii: false,
    };
    let matcher = Matcher::Bytes(sset);
    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher,
    };
    let result = searcher.find(&[]);
}

#[test]
fn test_find_bytes_match_single_byte_haystack() {
    let sset = SingleByteSet {
        sparse: vec![true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        dense: vec![1],
        complete: true,
        all_ascii: true,
    };
    let matcher = Matcher::Bytes(sset);
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher,
    };
    let result = searcher.find(&[1]);
}

#[test]
fn test_find_bytes_match_multiple_bytes_haystack() {
    let sset = SingleByteSet {
        sparse: vec![true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        dense: vec![2, 3],
        complete: true,
        all_ascii: true,
    };
    let matcher = Matcher::Bytes(sset);
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher,
    };
    let result = searcher.find(&[0, 1, 2, 3]);
}

#[test]
fn test_find_bytes_match_boundary_haystack() {
    let sset = SingleByteSet {
        sparse: vec![true, false, false, true],
        dense: vec![0, 3],
        complete: true,
        all_ascii: true,
    };
    let matcher = Matcher::Bytes(sset);
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher,
    };
    let result = searcher.find(&[3]);
}

#[test]
fn test_find_bytes_match_large_haystack() {
    let sset = SingleByteSet {
        sparse: vec![true, false, false, true],
        dense: vec![0, 3],
        complete: true,
        all_ascii: true,
    };
    let matcher = Matcher::Bytes(sset);
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher,
    };
    let haystack = vec![1; 10000]; // haystack of length 10000
    let result = searcher.find(&haystack);
}

