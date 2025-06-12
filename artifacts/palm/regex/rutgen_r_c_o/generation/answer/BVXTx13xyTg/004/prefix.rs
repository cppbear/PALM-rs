// Answer 0

#[test]
fn test_find_end_empty_haystack() {
    let searcher = LiteralSearcher::empty();
    let result = searcher.find_end(b"");
}

#[test]
fn test_find_end_no_match_short_haystack() {
    let lits = Literals::empty();
    let searcher = LiteralSearcher::prefixes(lits);
    let result = searcher.find_end(b"a");
}

#[test]
fn test_find_end_no_match_medium_haystack() {
    let lits = Literals::empty();
    let searcher = LiteralSearcher::prefixes(lits);
    let result = searcher.find_end(b"abcd");
}

#[test]
fn test_find_end_no_match_long_haystack() {
    let lits = Literals::empty();
    let searcher = LiteralSearcher::prefixes(lits);
    let result = searcher.find_end(b"abcdefghijklmno");
}

#[test]
fn test_find_end_no_match_max_length_haystack() {
    let lits = Literals::empty();
    let searcher = LiteralSearcher::prefixes(lits);
    let result = searcher.find_end(&[0u8; 256]);
} 

#[test]
fn test_find_end_lit_length_greater_than_haystack() {
    let lit_bytes = vec![b'x'; 300];
    let lits = Literals::from_vec(vec![Literal::from_bytes(&lit_bytes)]);
    let searcher = LiteralSearcher::prefixes(lits);
    let result = searcher.find_end(b"abcdef");
}

