// Answer 0

#[test]
fn test_find_start_empty_searcher() {
    let searcher = LiteralSearcher::empty();
    let haystack = b"hello";
    let result = searcher.find_start(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_start_matching_literal() {
    let literals = Literals::from(vec![b"hello".to_vec()]);
    let searcher = LiteralSearcher::prefixes(literals);
    let haystack = b"hello world";
    let result = searcher.find_start(haystack);
    assert_eq!(result, Some((0, 5)));
}

#[test]
fn test_find_start_no_matching_literal() {
    let literals = Literals::from(vec![b"world".to_vec()]);
    let searcher = LiteralSearcher::prefixes(literals);
    let haystack = b"hello there";
    let result = searcher.find_start(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_start_partial_match() {
    let literals = Literals::from(vec![b"hello".to_vec()]);
    let searcher = LiteralSearcher::prefixes(literals);
    let haystack = b"hell";
    let result = searcher.find_start(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_start_multiple_literals() {
    let literals = Literals::from(vec![b"hello".to_vec(), b"hi".to_vec()]);
    let searcher = LiteralSearcher::prefixes(literals);
    let haystack = b"hi there";
    let result = searcher.find_start(haystack);
    assert_eq!(result, Some((0, 2)));
}

