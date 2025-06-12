// Answer 0

#[test]
fn test_len_empty_matcher() {
    let literal_searcher = LiteralSearcher::empty();
    literal_searcher.len();
}

#[test]
fn test_len_empty_with_no_literals() {
    let lits = Literals::empty();
    let literal_searcher = LiteralSearcher::new(lits, Matcher::Empty);
    literal_searcher.len();
}

#[test]
fn test_len_empty_with_prefixes() {
    let lits = Literals::empty();
    let literal_searcher = LiteralSearcher::prefixes(lits);
    literal_searcher.len();
}

#[test]
fn test_len_empty_with_suffixes() {
    let lits = Literals::empty();
    let literal_searcher = LiteralSearcher::suffixes(lits);
    literal_searcher.len();
}

