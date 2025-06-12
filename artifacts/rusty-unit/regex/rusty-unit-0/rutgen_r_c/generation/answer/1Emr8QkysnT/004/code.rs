// Answer 0

#[test]
fn test_approximate_size_boyer_moore() {
    let pattern = b"example".to_vec();
    let boyer_moore_search = BoyerMooreSearch::new(pattern.clone());
    
    let matcher = Matcher::BoyerMoore(boyer_moore_search);
    let literal_searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(pattern.clone()),
        lcs: FreqyPacked::new(pattern.clone()),
        matcher,
    };

    let expected_size = boyer_moore_search.approximate_size();
    assert_eq!(literal_searcher.approximate_size(), expected_size);
}

#[test]
fn test_approximate_size_boyer_moore_empty_pattern() {
    let pattern = b"".to_vec();
    let boyer_moore_search = BoyerMooreSearch::new(pattern.clone());

    let matcher = Matcher::BoyerMoore(boyer_moore_search);
    let literal_searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(pattern.clone()),
        lcs: FreqyPacked::new(pattern.clone()),
        matcher,
    };

    let expected_size = boyer_moore_search.approximate_size();
    assert_eq!(literal_searcher.approximate_size(), expected_size);
}

