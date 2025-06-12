// Answer 0

#[test]
fn test_complete_false_is_empty_true() {
    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked {
            pat: vec![b'a'],
            char_len: 1,
            rare1: b'a',
            rare1i: 0,
            rare2: b'a',
            rare2i: 0,
        },
        lcs: FreqyPacked {
            pat: vec![b'a'],
            char_len: 1,
            rare1: b'a',
            rare1i: 0,
            rare2: b'a',
            rare2i: 0,
        },
        matcher: Matcher::Empty,
    };
    assert_eq!(searcher.complete(), false);
}

#[test]
fn test_complete_false_is_empty_false() {
    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked {
            pat: vec![b'a'],
            char_len: 1,
            rare1: b'a',
            rare1i: 0,
            rare2: b'a',
            rare2i: 0,
        },
        lcs: FreqyPacked {
            pat: vec![b'a'],
            char_len: 1,
            rare1: b'a',
            rare1i: 0,
            rare2: b'a',
            rare2i: 0,
        },
        matcher: Matcher::Bytes(SingleByteSet::new(vec![b'a'])),
    };
    assert_eq!(searcher.complete(), false);
}

