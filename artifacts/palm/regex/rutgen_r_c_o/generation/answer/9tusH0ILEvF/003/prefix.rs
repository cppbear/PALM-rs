// Answer 0

#[test]
fn test_check_match_success() {
    let pattern = vec![b'a', b'b', b'c'];
    let search = BoyerMooreSearch::new(pattern.clone());
    let haystack = vec![b'x', b'y', b'a', b'b', b'c'];
    let window_end = 5; // Matches the end of the haystack
    let result = search.check_match(&haystack, window_end);
}

#[test]
fn test_check_match_guard_condition() {
    let pattern = vec![b'x', b'y', b'z'];
    let search = BoyerMooreSearch::new(pattern.clone());
    let haystack = vec![b'a', b'b', b'x', b'y', b'z'];
    let window_end = 5; // Matches the end of the haystack
    let result = search.check_match(&haystack, window_end);
}

#[test]
fn test_check_match_partial_overlap() {
    let pattern = vec![b'a', b'b', b'c'];
    let search = BoyerMooreSearch::new(pattern.clone());
    let haystack = vec![b'b', b'a', b'b', b'c', b'a', b'b', b'c'];
    let window_end = 4; // Overlaps but matches the pattern
    let result = search.check_match(&haystack, window_end);
}

#[test]
#[should_panic]
fn test_check_match_out_of_bounds_window_end() {
    let pattern = vec![b'a', b'b'];
    let search = BoyerMooreSearch::new(pattern.clone());
    let haystack = vec![b'a', b'b', b'c'];
    let window_end = 3; // This will panic due to the out-of-bounds access
    let result = search.check_match(&haystack, window_end);
}

#[test]
fn test_check_match_no_match() {
    let pattern = vec![b'a', b'b', b'c'];
    let search = BoyerMooreSearch::new(pattern.clone());
    let haystack = vec![b'a', b'b', b'd'];
    let window_end = 3; // Should not match
    let result = search.check_match(&haystack, window_end);
}

