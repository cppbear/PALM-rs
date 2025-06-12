// Answer 0

#[test]
fn test_find_with_empty_haystack() {
    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(vec![b'a']),
        lcs: FreqyPacked::new(vec![b'a']),
        matcher: Matcher::Bytes(SingleByteSet {
            sparse: vec![false; 256],
            dense: vec![b'a'],
            complete: true,
            all_ascii: true,
        }),
    };
    assert_eq!(searcher.find(&[]), Some((0, 1)));
}

#[test]
fn test_find_with_non_matching_haystack() {
    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(vec![b'a']),
        lcs: FreqyPacked::new(vec![b'a']),
        matcher: Matcher::Bytes(SingleByteSet {
            sparse: vec![false; 256],
            dense: vec![b'a'],
            complete: true,
            all_ascii: true,
        }),
    };
    assert_eq!(searcher.find(b"bcdef"), None);
}

#[test]
fn test_find_with_matching_haystack() {
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![b'a']),
        lcs: FreqyPacked::new(vec![b'a']),
        matcher: Matcher::Bytes(SingleByteSet {
            sparse: vec![false; 256],
            dense: vec![b'a'],
            complete: true,
            all_ascii: true,
        }),
    };
    assert_eq!(searcher.find(b"abcdef"), Some((0, 1)));
}

#[test]
fn test_find_with_multiple_matches() {
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![b'a']),
        lcs: FreqyPacked::new(vec![b'a']),
        matcher: Matcher::Bytes(SingleByteSet {
            sparse: vec![false; 256],
            dense: vec![b'a'],
            complete: true,
            all_ascii: true,
        }),
    };
    assert_eq!(searcher.find(b"aabbac"), Some((0, 1)));
}

#[test]
fn test_find_with_characters_before_match() {
    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![b'a']),
        lcs: FreqyPacked::new(vec![b'a']),
        matcher: Matcher::Bytes(SingleByteSet {
            sparse: vec![false; 256],
            dense: vec![b'a'],
            complete: true,
            all_ascii: true,
        }),
    };
    assert_eq!(searcher.find(b"xaax"), Some((1, 2)));
}

