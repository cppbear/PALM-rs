// Answer 0

#[test]
fn test_find_start_empty_lit() {
    let searcher = LiteralSearcher::empty();
    let haystack = b"hello";
    assert_eq!(searcher.find_start(haystack), None);
}

#[test]
fn test_find_start_lit_longer_than_haystack() {
    let searcher = LiteralSearcher::prefixes(Literals::from(&[b"hello", b"world"]));
    let haystack = b"hi";
    assert_eq!(searcher.find_start(haystack), None);
}

#[test]
fn test_find_start_no_starting_match() {
    let searcher = LiteralSearcher::suffixes(Literals::from(&[b"test", b"sample"]));
    let haystack = b"hello";
    assert_eq!(searcher.find_start(haystack), None);
}

