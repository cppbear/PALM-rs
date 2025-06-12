// Answer 0

#[test]
fn test_len_empty_bytes() {
    let sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![],
        complete: false,
        all_ascii: true,
    };
    let matcher = Matcher::Bytes(sset);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    let result = searcher.len();
}

#[test]
fn test_len_small_bytes() {
    let sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: vec![1, 2, 3],
        complete: true,
        all_ascii: true,
    };
    let matcher = Matcher::Bytes(sset);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    let result = searcher.len();
}

#[test]
fn test_len_max_bytes() {
    let sset = SingleByteSet {
        sparse: vec![false; 256],
        dense: (0..256).collect(),
        complete: true,
        all_ascii: true,
    };
    let matcher = Matcher::Bytes(sset);
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    let result = searcher.len();
}

