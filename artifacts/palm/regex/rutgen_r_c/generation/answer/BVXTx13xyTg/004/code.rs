// Answer 0

#[test]
fn test_find_end_empty_literal_searcher() {
    let lits = Literals::empty();
    let searcher = LiteralSearcher::new(lits, Matcher::Empty);
    let result = searcher.find_end(b"any haystack");
    assert_eq!(result, None);
}

#[test]
fn test_find_end_no_matches() {
    let lits = Literals::from(vec![b"test".to_vec()]);
    let searcher = LiteralSearcher::new(lits, Matcher::FreqyPacked(FreqyPacked::new(vec![b"search".to_vec()])));
    let result = searcher.find_end(b"no match here");
    assert_eq!(result, None);
}

#[test]
fn test_find_end_matching_length_but_no_match() {
    let lits = Literals::from(vec![b"data".to_vec()]);
    let searcher = LiteralSearcher::new(lits, Matcher::FreqyPacked(FreqyPacked::new(vec![b"test".to_vec()])));
    let result = searcher.find_end(b"data");
    assert_eq!(result, None);
}

