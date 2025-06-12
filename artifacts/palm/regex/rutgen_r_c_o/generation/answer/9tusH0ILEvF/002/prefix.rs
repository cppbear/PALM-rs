// Answer 0

#[test]
fn test_check_match_false_guard_mismatch() {
    let pattern = vec![1, 2, 3];
    let haystack = vec![4, 5, 6, 7, 8]; // haystack does not contain the pattern
    let search = BoyerMooreSearch::new(pattern);
    let window_end = 4; // points to element 7
    let result = search.check_match(&haystack, window_end);
}

#[test]
fn test_check_match_false_pattern_mismatch() {
    let pattern = vec![1, 2, 3];
    let haystack = vec![1, 2, 4, 3, 5]; // haystack contains similar elements but does not match the pattern
    let search = BoyerMooreSearch::new(pattern);
    let window_end = 3; // points to element 4, we will check pattern at position 1
    let result = search.check_match(&haystack, window_end);
}

#[test]
fn test_check_match_false_pattern_edge_case() {
    let pattern = vec![255]; // pattern of length 1
    let haystack = vec![1, 2, 3]; // haystack does not contain the pattern value
    let search = BoyerMooreSearch::new(pattern);
    let window_end = 1; // checking the first element
    let result = search.check_match(&haystack, window_end);
} 

#[test]
fn test_check_match_false_haystack_larger_than_pattern() {
    let pattern = vec![1, 2, 3, 4];
    let haystack = vec![5, 6, 7, 8]; // unrelated values
    let search = BoyerMooreSearch::new(pattern);
    let window_end = 4; // checks just past the end of the haystack
    let result = search.check_match(&haystack, window_end);
}

#[test]
fn test_check_match_false_partial_match() {
    let pattern = vec![1, 2, 3];
    let haystack = vec![1, 2, 3, 4, 5]; // full match expected at window_end later
    let search = BoyerMooreSearch::new(pattern);
    let window_end = 4; // should find it at position 1-3, but checking 4
    let result = search.check_match(&haystack, window_end);
}

