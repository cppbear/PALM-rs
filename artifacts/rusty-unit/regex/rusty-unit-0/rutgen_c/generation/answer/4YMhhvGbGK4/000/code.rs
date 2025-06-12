// Answer 0

#[test]
fn test_complete_when_complete_and_not_empty() {
    let lcp = FreqyPacked {
        pat: b"abc".to_vec(),
        char_len: 3,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    let lcs = FreqyPacked {
        pat: b"abc".to_vec(),
        char_len: 3,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    let matcher = Matcher::Bytes(SingleByteSet::new());
    let searcher = LiteralSearcher {
        complete: true,
        lcp,
        lcs,
        matcher,
    };
    
    assert!(searcher.complete());
}

#[test]
fn test_complete_when_complete_but_empty() {
    let lcp = FreqyPacked {
        pat: b"abc".to_vec(),
        char_len: 3,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    let lcs = FreqyPacked {
        pat: b"abc".to_vec(),
        char_len: 3,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    let matcher = Matcher::Bytes(SingleByteSet::new());
    let searcher = LiteralSearcher {
        complete: true,
        lcp,
        lcs,
        matcher,
    };
    
    assert!(!searcher.is_empty());
    assert!(searcher.complete());
}

#[test]
fn test_complete_when_incomplete() {
    let lcp = FreqyPacked {
        pat: b"abc".to_vec(),
        char_len: 3,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    let lcs = FreqyPacked {
        pat: b"abc".to_vec(),
        char_len: 3,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    let matcher = Matcher::Bytes(SingleByteSet::new());
    let searcher = LiteralSearcher {
        complete: false,
        lcp,
        lcs,
        matcher,
    };

    assert!(!searcher.complete());
}

#[test]
fn test_complete_when_complete_and_empty() {
    let lcp = FreqyPacked {
        pat: vec![],
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };
    let lcs = FreqyPacked {
        pat: vec![],
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };
    let matcher = Matcher::Empty;
    let searcher = LiteralSearcher {
        complete: true,
        lcp,
        lcs,
        matcher,
    };

    assert!(!searcher.complete());
}

