// Answer 0

#[test]
fn test_lcp_empty() {
    let searcher = LiteralSearcher::empty();
    let _result = searcher.lcp();
}

#[test]
fn test_lcp_single_byte_pattern() {
    let lits = Literals::from(vec![b'a'.to_vec()]);
    let searcher = LiteralSearcher::prefixes(lits);
    let _result = searcher.lcp();
}

#[test]
fn test_lcp_multiple_byte_pattern() {
    let lits = Literals::from(vec![b"abc".to_vec(), b"abcd".to_vec()]);
    let searcher = LiteralSearcher::prefixes(lits);
    let _result = searcher.lcp();
}

#[test]
fn test_lcp_with_rare_bytes() {
    let mut pat = vec![b'a', b'b', b'c', b'd', b'e'];
    let lcp = FreqyPacked {
        pat: pat.clone(),
        char_len: 5,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    let matcher = Matcher::FreqyPacked(lcp);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    let _result = searcher.lcp();
}

#[test]
fn test_lcp_with_255_byte_pattern() {
    let pat = (0..255).map(|i| i as u8).collect::<Vec<u8>>();
    let lcp = FreqyPacked {
        pat: pat.clone(),
        char_len: 255,
        rare1: pat[0],
        rare1i: 0,
        rare2: pat[1],
        rare2i: 1,
    };
    let matcher = Matcher::FreqyPacked(lcp);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    let _result = searcher.lcp();
}

