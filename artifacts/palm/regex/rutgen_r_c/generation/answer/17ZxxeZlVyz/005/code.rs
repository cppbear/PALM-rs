// Answer 0

#[test]
fn test_find_equal_length_haystack_and_pattern() {
    let pattern = b"abc";
    let haystack = b"abc"; // haystack.len() == pattern.len()

    let bms = BoyerMooreSearch::new(pattern.to_vec());
    let result = bms.find(haystack);

    assert_eq!(result, None); // Expected None since they are equal in length.
}

#[test]
fn test_find_haystack_exceeds_short_circuit() {
    let pattern = b"abc";
    let haystack = b"exampleabc"; // haystack.len() > short_circut

    let bms = BoyerMooreSearch::new(pattern.to_vec());
    let result = bms.find(haystack);

    assert_eq!(result, Some(7)); // Pattern starts at index 7.
}

#[test]
fn test_find_haystack_meets_skip_loop_constraints() {
    let pattern = b"abc";
    let haystack = b"xyzabcbrsd"; // haystack.len() > short_circut

    let bms = BoyerMooreSearch::new(pattern.to_vec());
    let result = bms.find(haystack);
    
    assert_eq!(result, Some(3)); // Pattern starts at index 3.
}

#[test]
fn test_find_window_end_equal_backstop() {
    let pattern = b"abc";
    let haystack = b"abcdefghij"; // haystack.len() > short_circut

    let bms = BoyerMooreSearch::new(pattern.to_vec());
    let result = bms.find(haystack);

    assert_eq!(result, Some(0)); // Pattern found at index 0.
} 

#[test]
fn test_find_window_end_not_less_than_haystack_len() {
    let pattern = b"abc";
    let haystack = b"abcxyz"; // Ensure the pattern exists.

    let bms = BoyerMooreSearch::new(pattern.to_vec());
    let result = bms.find(haystack);

    assert_eq!(result, Some(0)); // Pattern found at index 0.
}

