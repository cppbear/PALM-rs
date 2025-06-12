// Answer 0

#[test]
fn test_find_end_with_non_matching_lit() {
    let searcher = LiteralSearcher::empty();
    let haystack = b"hello world";
    let result = searcher.find_end(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_end_with_lit_longer_than_haystack() {
    let lit = FreqyPacked {
        pat: b"hello world!".to_vec(),
        char_len: 12,
        rare1: b'!'.to_owned(),
        rare1i: 11,
        rare2: b'h'.to_owned(),
        rare2i: 0,
    };
    
    let searcher = LiteralSearcher {
        complete: true,
        lcp: lit.clone(),
        lcs: lit.clone(),
        matcher: Matcher::FreqyPacked(lit),
    };
    
    let haystack = b"hello";
    let result = searcher.find_end(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_end_with_empty_lit() {
    let searcher = LiteralSearcher::empty();
    let haystack = b"";
    let result = searcher.find_end(haystack);
    assert_eq!(result, None);
}

