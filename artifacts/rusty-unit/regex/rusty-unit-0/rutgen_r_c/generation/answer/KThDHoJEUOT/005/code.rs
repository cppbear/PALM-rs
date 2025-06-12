// Answer 0

#[test]
fn test_skip_loop_with_progress() {
    let pattern = vec![b'a', b'b', b'c'];
    let haystack = b"abcabcabcabcabc";
    let mut search = BoyerMooreSearch::new(pattern);

    // Set the window_end to the last index of "abc" found in the haystack.
    let window_end = 14;  // index of 'c' in "abcabcabcabcabc"
    let backstop = haystack.len(); // backstop is the length of the haystack

    // Test if we get Some(window_end)
    let result = search.skip_loop(haystack, window_end, backstop);
    assert_eq!(result, Some(14));
}

#[test]
fn test_skip_loop_no_progress() {
    let pattern = vec![b'a', b'b', b'c'];
    let haystack = b"abcabcabcabcabc";
    let mut search = BoyerMooreSearch::new(pattern);

    // Set window_end to start of haystack
    let window_end = 0;  // index of 'a' in "abcabcabcabcabc"
    let backstop = haystack.len(); // backstop is the length of the haystack

    // Skip might result in zero progress so we'd expect to have progress only by guard
    let result = search.skip_loop(haystack, window_end, backstop);
    assert_eq!(result, Some(0)); // Should return the start if no skip is applied
}

#[test]
fn test_skip_loop_reaches_backstop() {
    let pattern = vec![b'a', b'b', b'c'];
    let haystack = b"abcabcabcabcabc";
    let mut search = BoyerMooreSearch::new(pattern);

    // Set the window_end to a position where we will hit backstop
    let window_end = 12;  // near the end of the haystack
    let backstop = haystack.len(); // backstop is the length of the haystack

    // Expect to hit backstop with some skips
    let result = search.skip_loop(haystack, window_end, backstop);
    assert_eq!(result, Some(14)); // We should find the last 'c'
}

#[test]
#[should_panic] // This test case checks if proper panic situations are handled
fn test_skip_loop_panic_on_invalid_index() {
    let pattern = vec![b'a', b'b', b'c'];
    let haystack = b"abcabcabcabcabc";
    let mut search = BoyerMooreSearch::new(pattern);

    // Attempt to use a window_end out of bounds
    let window_end = 20; // invalid index
    let backstop = haystack.len();

    // This should panic due to index out of bounds
    search.skip_loop(haystack, window_end, backstop);
}

