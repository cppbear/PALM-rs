// Answer 0

#[test]
fn test_find_end_with_empty_literals() {
    let searcher = LiteralSearcher::empty();
    let result = searcher.find_end(b"sample haystack");
    assert_eq!(result, None);
}

#[test]
fn test_find_end_with_non_matching_literal_length() {
    let literals = Literals::from(vec!["test".as_bytes().to_vec()]);
    let searcher = LiteralSearcher::suffixes(literals);
    let result = searcher.find_end(b"short");
    assert_eq!(result, None);
}

#[test]
fn test_find_end_with_non_matching_literal() {
    let literals = Literals::from(vec!["sample".as_bytes().to_vec()]);
    let searcher = LiteralSearcher::suffixes(literals);
    let result = searcher.find_end(b"example haystack");
    assert_eq!(result, None);
}

#[test]
fn test_find_end_with_matching_length_but_not_matching() {
    let literals = Literals::from(vec!["matcher".as_bytes().to_vec()]);
    let searcher = LiteralSearcher::suffixes(literals);
    let result = searcher.find_end(b"sample haystack");
    assert_eq!(result, None);
}

