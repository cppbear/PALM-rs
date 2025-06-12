// Answer 0

#[test]
fn test_find_exact_match() {
    let pattern = b"hello".to_vec();
    let haystack = b"hello".to_vec();
    let searcher = BoyerMooreSearch::new(pattern);
    let result = searcher.find(&haystack);
}

#[test]
fn test_find_edge_case_short_circuit() {
    let pattern = b"pattern".to_vec();
    let haystack = vec![b'x'; (10 + 2) * pattern.len() - 1]; // using some filler
    let searcher = BoyerMooreSearch::new(pattern);
    let result = searcher.find(&haystack);
}

#[test]
fn test_find_panic_conditions() {
    let pattern = b"abc".to_vec();
    let haystack = b"xxxxabcxxxx".to_vec();
    let searcher = BoyerMooreSearch::new(pattern);
    let result = searcher.find(&haystack);
}

#[test]
fn test_find_multiple_occurrences() {
    let pattern = b"test".to_vec();
    let haystack = b"this is a test for testing the test case".to_vec();
    let searcher = BoyerMooreSearch::new(pattern);
    let result = searcher.find(&haystack);
}

#[test]
fn test_find_no_match() {
    let pattern = b"nope".to_vec();
    let haystack = b"this will not contain the pattern".to_vec();
    let searcher = BoyerMooreSearch::new(pattern);
    let result = searcher.find(&haystack);
}

