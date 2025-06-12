// Answer 0

#[test]
fn test_iter_empty_matcher() {
    let searcher = LiteralSearcher::empty();
    let iter_result = searcher.iter();
}

#[test]
fn test_iter_boyer_moore() {
    let searcher = LiteralSearcher::new(
        Literals::empty(),
        Matcher::BoyerMoore(BoyerMooreSearch {
            pattern: b"test".to_vec(),
            skip_table: vec![0; 256],
            guard: b'a',
            guard_reverse_idx: 0,
            md2_shift: 1,
        }),
    );
    let iter_result = searcher.iter();
}

#[test]
fn test_iter_freqy_packed() {
    let searcher = LiteralSearcher::new(
        Literals::empty(),
        Matcher::FreqyPacked(FreqyPacked {
            pat: b"example".to_vec(),
            char_len: 7,
            rare1: b'e',
            rare1i: 0,
            rare2: b'x',
            rare2i: 1,
        }),
    );
    let iter_result = searcher.iter();
}

#[test]
fn test_iter_teddy_ssse3() {
    let searcher = LiteralSearcher::new(
        Literals::empty(),
        Matcher::TeddySSSE3(TeddySSSE3 {
            vb: SSSE3VectorBuilder::default(),
            pats: vec![b"match".to_vec()],
            ac: FullAcAutomaton::new(vec![b"match".to_vec()]),
            buckets: vec![vec![0]],
            masks: Masks::default(),
        }),
    );
    let iter_result = searcher.iter();
}

#[test]
fn test_iter_teddy_avx2() {
    let searcher = LiteralSearcher::new(
        Literals::empty(),
        Matcher::TeddyAVX2(TeddyAVX2 {
            vb: AVX2VectorBuilder::default(),
            pats: vec![b"example".to_vec()],
            ac: FullAcAutomaton::new(vec![b"example".to_vec()]),
            buckets: vec![vec![0]],
            masks: Masks::default(),
        }),
    );
    let iter_result = searcher.iter();
}

#[test]
fn test_iter_bytes() {
    let searcher = LiteralSearcher::new(
        Literals::empty(),
        Matcher::Bytes(SingleByteSet {
            sparse: vec![false; 256],
            dense: b"abc".to_vec(),
            complete: false,
            all_ascii: true,
        }),
    );
    let iter_result = searcher.iter();
}

