// Answer 0

#[test]
fn test_find_start_with_haystack_longer_than_lit() {
    let lit = Literals::from_vec(vec![vec![1, 2, 3, 4]]);
    let searcher = LiteralSearcher::prefixes(lit);
    let haystack = vec![1, 2, 3];
    let _ = searcher.find_start(&haystack);
}

#[test]
fn test_find_start_with_empty_haystack() {
    let lit = Literals::from_vec(vec![vec![1]]);
    let searcher = LiteralSearcher::prefixes(lit);
    let haystack = vec![];
    let _ = searcher.find_start(&haystack);
}

#[test]
fn test_find_start_with_non_matching_lit() {
    let lit = Literals::from_vec(vec![vec![5, 6, 7, 8]]);
    let searcher = LiteralSearcher::prefixes(lit);
    let haystack = vec![1, 2, 3, 4];
    let _ = searcher.find_start(&haystack);
}

