// Answer 0

#[test]
fn test_iter_with_bytes_matcher() {
    let sset = SingleByteSet {
        sparse: vec![true, false, true, false],
        dense: vec![b'a', b'b', b'c', b'd'],
        complete: true,
        all_ascii: true,
    };
    
    let matcher = Matcher::Bytes(sset.clone());
    
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    
    let iter = searcher.iter();
}

#[test]
fn test_iter_with_bytes_matcher_empty_dense() {
    let sset = SingleByteSet {
        sparse: vec![false, false, false, false],
        dense: vec![],
        complete: false,
        all_ascii: true,
    };
    
    let matcher = Matcher::Bytes(sset.clone());
    
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    
    let iter = searcher.iter();
}

#[test]
fn test_iter_with_bytes_matcher_single_entry() {
    let sset = SingleByteSet {
        sparse: vec![true, false],
        dense: vec![b'a'],
        complete: true,
        all_ascii: true,
    };
    
    let matcher = Matcher::Bytes(sset.clone());
    
    let searcher = LiteralSearcher::new(Literals::empty(), matcher);
    
    let iter = searcher.iter();
}

#[test]
fn test_iter_with_bytes_matcher_multiple_entries() {
    let sset = SingleByteSet {
        sparse: vec![true, true, false],
        dense: vec![b'a', b'b'],
        complete: true,
        all_ascii: true,
    };

    let matcher = Matcher::Bytes(sset.clone());

    let searcher = LiteralSearcher::new(Literals::empty(), matcher);

    let iter = searcher.iter();
}

