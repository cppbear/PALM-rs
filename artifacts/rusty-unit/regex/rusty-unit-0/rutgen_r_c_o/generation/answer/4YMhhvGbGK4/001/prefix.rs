// Answer 0

#[test]
fn test_complete_with_non_empty_freqy_packed() {
    let lcp = FreqyPacked {
        pat: vec![b'a'; 1],
        char_len: 1,
        rare1: b'a',
        rare1i: 0,
        rare2: b'a',
        rare2i: 0,
    };
    let lcs = FreqyPacked {
        pat: vec![b'z'; 1],
        char_len: 1,
        rare1: b'z',
        rare1i: 0,
        rare2: b'z',
        rare2i: 0,
    };
    let litts = Literals::new(vec![]);
    let matcher = Matcher::FreqyPacked(lcp.clone());
    let searcher = LiteralSearcher::new(litts, matcher);

    let result = searcher.complete();
}

#[test]
fn test_complete_with_non_empty_boyer_moore() {
    let lcp = FreqyPacked {
        pat: vec![b'x'; 2],
        char_len: 2,
        rare1: b'x',
        rare1i: 0,
        rare2: b'x',
        rare2i: 1,
    };
    let lcs = FreqyPacked {
        pat: vec![b'y'; 2],
        char_len: 2,
        rare1: b'y',
        rare1i: 0,
        rare2: b'y',
        rare2i: 1,
    };
    let litts = Literals::new(vec![]);
    let matcher = Matcher::BoyerMoore(BoyerMooreSearch::new());
    let searcher = LiteralSearcher::new(litts, matcher);

    let result = searcher.complete();
}

#[test]
fn test_complete_with_non_empty_ac() {
    let lcp = FreqyPacked {
        pat: vec![b'b'; 3],
        char_len: 3,
        rare1: b'b',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    let lcs = FreqyPacked {
        pat: vec![b'c'; 3],
        char_len: 3,
        rare1: b'c',
        rare1i: 0,
        rare2: b'c',
        rare2i: 1,
    };
    let litts = Literals::new(vec![]);
    let matcher = Matcher::AC(FullAcAutomaton::new());
    let searcher = LiteralSearcher::new(litts, matcher);

    let result = searcher.complete();
}

