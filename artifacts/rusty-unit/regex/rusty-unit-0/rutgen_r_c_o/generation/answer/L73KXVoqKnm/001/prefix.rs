// Answer 0

#[test]
fn test_is_empty_with_empty_literal_searcher() {
    let lits = Literals::empty();
    let matcher = Matcher::Empty;
    let searcher = LiteralSearcher::new(lits, matcher);
    searcher.is_empty();
}

#[test]
fn test_is_empty_with_empty_freqy_packed() {
    let freqy_packed = FreqyPacked {
        pat: Vec::new(),
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };
    let matcher = Matcher::FreqyPacked(freqy_packed);
    let lits = Literals::empty();
    let searcher = LiteralSearcher::new(lits, matcher);
    searcher.is_empty();
}

#[test]
fn test_is_empty_with_empty_bytes() {
    let single_byte_set = SingleByteSet::new(Vec::new());
    let matcher = Matcher::Bytes(single_byte_set);
    let lits = Literals::empty();
    let searcher = LiteralSearcher::new(lits, matcher);
    searcher.is_empty();
}

