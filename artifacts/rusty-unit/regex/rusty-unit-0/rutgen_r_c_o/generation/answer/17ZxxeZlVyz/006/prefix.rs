// Answer 0

#[test]
fn test_find_exact_match() {
    let pattern = vec![1, 2, 3, 4];
    let haystack = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4];
    let bms = BoyerMooreSearch::new(pattern);
    let _ = bms.find(&haystack);
}

#[test]
fn test_find_multiple_occurrences() {
    let pattern = vec![2, 3, 4];
    let haystack = vec![1, 2, 3, 4, 5, 2, 3, 4, 6, 7, 2, 3, 4, 8, 9];
    let bms = BoyerMooreSearch::new(pattern);
    let _ = bms.find(&haystack);
}

#[test]
fn test_find_no_match_with_full_length() {
    let pattern = vec![10, 11, 12, 13];
    let haystack = vec![1, 2, 3, 4, 5, 6, 10, 11, 12, 13, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let bms = BoyerMooreSearch::new(pattern);
    let _ = bms.find(&haystack);
}

#[test]
fn test_find_pattern_at_start() {
    let pattern = vec![1, 2, 3, 4];
    let haystack = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4];
    let bms = BoyerMooreSearch::new(pattern);
    let _ = bms.find(&haystack);
}

#[test]
fn test_find_pattern_at_end() {
    let pattern = vec![8, 9, 1, 2];
    let haystack = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4];
    let bms = BoyerMooreSearch::new(pattern);
    let _ = bms.find(&haystack);
}

