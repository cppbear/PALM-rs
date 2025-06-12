// Answer 0

#[test]
fn test_find_end_with_empty_haystack() {
    let searcher = LiteralSearcher::empty();
    let haystack: &[u8] = b"";
    let _ = searcher.find_end(haystack);
}

#[test]
fn test_find_end_with_short_haystack_and_long_literal() {
    let literals = Literals::from(vec![Literal::from("longliteral")]);
    let searcher = LiteralSearcher::prefixes(literals);
    let haystack: &[u8] = b"short";
    let _ = searcher.find_end(haystack);
}

#[test]
fn test_find_end_with_haystack_smaller_than_literal() {
    let literals = Literals::from(vec![Literal::from("testliteral")]);
    let searcher = LiteralSearcher::suffixes(literals);
    let haystack: &[u8] = b"tiny";
    let _ = searcher.find_end(haystack);
}

#[test]
fn test_find_end_with_no_matching_lit() {
    let literals = Literals::from(vec![Literal::from("notfound")]);
    let searcher = LiteralSearcher::suffixes(literals);
    let haystack: &[u8] = b"anothertext";
    let _ = searcher.find_end(haystack);
}

#[test]
fn test_find_end_with_exact_match() {
    let literals = Literals::from(vec![Literal::from("exactmatch")]);
    let searcher = LiteralSearcher::suffixes(literals);
    let haystack: &[u8] = b"this is an exactmatch";
    let _ = searcher.find_end(haystack);
}

#[test]
fn test_find_end_with_long_haystack_and_no_match() {
    let literals = Literals::from(vec![Literal::from("notpresent")]);
    let searcher = LiteralSearcher::suffixes(literals);
    let haystack: &[u8] = b"a long text without matches whatsoever";
    let _ = searcher.find_end(haystack);
}

