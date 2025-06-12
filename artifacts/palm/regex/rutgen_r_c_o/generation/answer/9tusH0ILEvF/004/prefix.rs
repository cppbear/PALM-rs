// Answer 0

#[test]
fn test_check_match_valid_case() {
    let pattern = vec![b'a'];
    let search = BoyerMooreSearch::new(pattern.clone());
    let haystack = vec![b'a'];

    let window_end = 1; // Only position that satisfies the constraints
    let result = search.check_match(&haystack, window_end);
}

#[test]
#[should_panic]
fn test_check_match_guard_fail() {
    let pattern = vec![b'a'];
    let search = BoyerMooreSearch::new(pattern.clone());
    let haystack = vec![b'b'];

    let window_end = 1; // Should panic, guard check fails
    let result = search.check_match(&haystack, window_end);
}

#[test]
fn test_check_match_empty_case() {
    let pattern: Vec<u8> = vec![b'a'];
    let search = BoyerMooreSearch::new(pattern.clone());
    let haystack = vec![b'a'];

    let window_end = 1; // Only position that satisfies the constraints
    let result = search.check_match(&haystack, window_end);
}

