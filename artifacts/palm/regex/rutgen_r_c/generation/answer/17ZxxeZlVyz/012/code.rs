// Answer 0

#[test]
fn test_find_on_exact_length_haystack() {
    let pattern = b"abc".to_vec();
    let haystack = b"abc".to_vec(); // haystack.len() == pattern.len()
    let search = BoyerMooreSearch::new(pattern);

    assert_eq!(search.find(&haystack), Some(0)); // Expecting to find the pattern at the start
}

#[test]
fn test_find_on_short_circuit_length_haystack() {
    let pattern = b"abc".to_vec();
    let haystack = vec![b'a', b'b', b'c', b'd', b'e', b'f']; // Ensure haystack.len() > short_circut
    let search = BoyerMooreSearch::new(pattern);

    assert_eq!(search.find(&haystack), Some(0)); // Expecting to find the pattern at the start
}

#[test]
fn test_find_when_skip_is_zero_and_no_match() {
    let pattern = b"xyz".to_vec();
    let haystack = b"abc".to_vec(); // Non-matching haystack
    let search = BoyerMooreSearch::new(pattern);

    assert_eq!(search.find(&haystack), None); // Should not find the pattern
}

#[test]
fn test_find_when_window_end_at_length() {
    let pattern = b"abc".to_vec();
    let haystack = b"xyzabc".to_vec(); // Make sure haystack is longer; triggering case where window_end == haystack.len()
    let search = BoyerMooreSearch::new(pattern);

    let result = search.find(&haystack);
    assert!(result.is_none()); // Should not find the pattern, as it needs to check previous index
}

#[test]
fn test_find_on_edge_case_skip_zero() {
    let pattern = b"b".to_vec();
    let haystack = b"aaab".to_vec(); // haystack where skip == 0 for some indexes
    let search = BoyerMooreSearch::new(pattern);

    assert_eq!(search.find(&haystack), Some(3)); // Expect to find the single character at the end
}

