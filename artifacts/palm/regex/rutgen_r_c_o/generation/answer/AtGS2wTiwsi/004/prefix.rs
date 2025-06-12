// Answer 0

#[test]
fn test_find_start_empty_haystack() {
    let searcher = LiteralSearcher::empty();
    let haystack: &[u8] = b"";
    let result = searcher.find_start(haystack);
}

#[test]
fn test_find_start_no_literals() {
    let literals = Literals::empty();
    let searcher = LiteralSearcher::new(literals, Matcher::Empty);
    let haystack: &[u8] = b"";
    let result = searcher.find_start(haystack);
}

