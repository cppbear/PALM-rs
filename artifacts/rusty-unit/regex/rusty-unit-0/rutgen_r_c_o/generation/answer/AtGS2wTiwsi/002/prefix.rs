// Answer 0

#[test]
fn test_find_start_case1() {
    let searcher = LiteralSearcher::prefixes(Literals::from_vec(vec![b"test".to_vec()]));
    let haystack = b"test";
    searcher.find_start(haystack);
}

#[test]
fn test_find_start_case2() {
    let searcher = LiteralSearcher::prefixes(Literals::from_vec(vec![b"example".to_vec()]));
    let haystack = b"example";
    searcher.find_start(haystack);
}

#[test]
fn test_find_start_case3() {
    let searcher = LiteralSearcher::prefixes(Literals::from_vec(vec![b"abcd".to_vec()]));
    let haystack = b"abcd";
    searcher.find_start(haystack);
}

#[test]
fn test_find_start_case4() {
    let searcher = LiteralSearcher::prefixes(Literals::from_vec(vec![b"hello".to_vec()]));
    let haystack = b"hello";
    searcher.find_start(haystack);
}

#[test]
fn test_find_start_case5() {
    let searcher = LiteralSearcher::prefixes(Literals::from_vec(vec![b"rustacean".to_vec()]));
    let haystack = b"rustacean";
    searcher.find_start(haystack);
}

