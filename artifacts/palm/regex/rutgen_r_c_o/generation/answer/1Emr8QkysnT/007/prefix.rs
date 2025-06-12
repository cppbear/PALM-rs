// Answer 0

#[test]
fn test_approximate_size_empty() {
    let literals = Literals::empty();
    let searcher = LiteralSearcher::new(literals, Matcher::Empty);
    searcher.approximate_size();
}

#[test]
fn test_approximate_size_bytes() {
    let bytes_set = SingleByteSet::new();
    let matcher = Matcher::Bytes(bytes_set);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    searcher.approximate_size();
}

#[test]
fn test_approximate_size_freqy_packed() {
    let freqy_packed = FreqyPacked::new(vec![b'a', b'b', b'c']);
    let matcher = Matcher::FreqyPacked(freqy_packed);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    searcher.approximate_size();
}

#[test]
fn test_approximate_size_boyer_moore() {
    let boyer_moore = BoyerMooreSearch::new(vec![b'a', b'b', b'c']);
    let matcher = Matcher::BoyerMoore(boyer_moore);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    searcher.approximate_size();
}

#[test]
fn test_approximate_size_aho_corasick() {
    let patterns = vec![vec![b'a', b'b', b'c']];
    let ac = FullAcAutomaton::new(patterns);
    let matcher = Matcher::AC(ac);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    searcher.approximate_size();
}

#[test]
fn test_approximate_size_teddy_ssse3() {
    let teddy = TeddySSSE3 { vb: SSSE3VectorBuilder::new(), pats: vec![vec![b'a', b'b', b'c']], ac: FullAcAutomaton::new(vec![]), buckets: vec![], masks: Masks::new() };
    let matcher = Matcher::TeddySSSE3(teddy);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    searcher.approximate_size();
}

#[test]
fn test_approximate_size_teddy_avx2() {
    let teddy = TeddyAVX2 { vb: AVX2VectorBuilder::new(), pats: vec![vec![b'a', b'b', b'c']], ac: FullAcAutomaton::new(vec![]), buckets: vec![], masks: Masks::new() };
    let matcher = Matcher::TeddyAVX2(teddy);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    searcher.approximate_size();
}

