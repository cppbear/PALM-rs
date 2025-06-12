// Answer 0

#[test]
fn test_approximate_size_freqy_packed() {
    let lcp_pat = vec![b'a', b'b', b'c', b'd', b'e'];
    let lcs_pat = vec![b'x', b'y', b'z', b'w', b'v'];

    let lcp = FreqyPacked {
        pat: lcp_pat.clone(),
        char_len: 5,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };

    let lcs = FreqyPacked {
        pat: lcs_pat.clone(),
        char_len: 5,
        rare1: b'x',
        rare1i: 0,
        rare2: b'y',
        rare2i: 1,
    };

    let matcher = Matcher::FreqyPacked(lcp.clone());
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);

    let size = searcher.approximate_size();
}

#[test]
fn test_approximate_size_freqy_packed_edge_case() {
    let lcp_pat = vec![b'a'];
    let lcs_pat = vec![b'z'];

    let lcp = FreqyPacked {
        pat: lcp_pat.clone(),
        char_len: 1,
        rare1: b'a',
        rare1i: 0,
        rare2: b'a',
        rare2i: 0,
    };

    let lcs = FreqyPacked {
        pat: lcs_pat.clone(),
        char_len: 1,
        rare1: b'z',
        rare1i: 0,
        rare2: b'z',
        rare2i: 0,
    };

    let matcher = Matcher::FreqyPacked(lcp.clone());
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);

    let size = searcher.approximate_size();
}

#[test]
fn test_approximate_size_freqy_packed_max_case() {
    let lcp_pat = vec![b'a'; 1024];
    let lcs_pat = vec![b'z'; 1024];

    let lcp = FreqyPacked {
        pat: lcp_pat.clone(),
        char_len: 128,
        rare1: b'a',
        rare1i: 0,
        rare2: b'a',
        rare2i: 0,
    };

    let lcs = FreqyPacked {
        pat: lcs_pat.clone(),
        char_len: 128,
        rare1: b'z',
        rare1i: 0,
        rare2: b'z',
        rare2i: 0,
    };

    let matcher = Matcher::FreqyPacked(lcp.clone());
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);

    let size = searcher.approximate_size();
}

