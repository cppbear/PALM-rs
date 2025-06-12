// Answer 0

#[test]
fn test_approximate_size_bytes_complete_ascii() {
    let sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'a'; 256], // 256 bytes of 'a'
        complete: true,
        all_ascii: true,
    };

    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher: Matcher::Bytes(sset),
    };

    let _ = searcher.approximate_size();
}

#[test]
fn test_approximate_size_bytes_complete_non_ascii() {
    let sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'\xFF'; 128], // 128 bytes of non-ASCII
        complete: true,
        all_ascii: false,
    };

    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher: Matcher::Bytes(sset),
    };

    let _ = searcher.approximate_size();
}

#[test]
fn test_approximate_size_bytes_incomplete_ascii() {
    let sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'b'; 64], // 64 bytes of 'b'
        complete: false,
        all_ascii: true,
    };

    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher: Matcher::Bytes(sset),
    };

    let _ = searcher.approximate_size();
}

#[test]
fn test_approximate_size_bytes_incomplete_non_ascii() {
    let sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![b'\x00'; 32], // 32 bytes of null character
        complete: false,
        all_ascii: false,
    };

    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher: Matcher::Bytes(sset),
    };

    let _ = searcher.approximate_size();
}

#[test]
fn test_approximate_size_bytes_empty_dense() {
    let sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![], // 0 bytes
        complete: true,
        all_ascii: true,
    };

    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher: Matcher::Bytes(sset),
    };

    let _ = searcher.approximate_size();
}

