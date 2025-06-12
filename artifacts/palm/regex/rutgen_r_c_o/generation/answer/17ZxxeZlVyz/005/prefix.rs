// Answer 0

#[test]
fn test_find_with_bound_haystack_equals_pattern() {
    let pattern = vec![1, 2, 3, 4, 5];
    let haystack = vec![1, 2, 3, 4, 5];
    let bms = BoyerMooreSearch::new(pattern);
    bms.find(&haystack);
}

#[test]
fn test_find_with_haystack_greater_than_short_circuit() {
    let pattern = vec![0, 1, 2, 3, 4, 5];
    let haystack = vec![0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 
                        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 1, 2, 3, 4];
    let bms = BoyerMooreSearch::new(pattern);
    bms.find(&haystack);
}

#[test]
fn test_find_with_matching_skip_loop() {
    let pattern = vec![2, 3, 4];
    let haystack = vec![0, 1, 2, 2, 3, 4, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 
                        2, 3, 4, 5, 6, 7, 8, 9];
    let bms = BoyerMooreSearch::new(pattern);
    bms.find(&haystack);
}

#[test]
fn test_find_with_window_end_equals_backstop() {
    let pattern = vec![5, 6, 7];
    let haystack = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 5, 6, 7, 8, 9, 
                        0, 1, 2, 3, 4, 5, 6, 7, 8];
    let bms = BoyerMooreSearch::new(pattern);
    bms.find(&haystack);
}

#[test]
fn test_find_if_window_end_equals_haystack_length() {
    let pattern = vec![10, 11];
    let haystack = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let bms = BoyerMooreSearch::new(pattern);
    bms.find(&haystack);
}

