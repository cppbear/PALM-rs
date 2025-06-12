// Answer 0

#[test]
fn test_skip_loop_found() {
    let pattern = b"abc".to_vec();
    let searcher = BoyerMooreSearch::new(pattern);
    
    let haystack = b"zzzabcxyz";
    let window_end = 6; // 'x' in the string "zzzabcxyz"
    let backstop = 10; // length of the haystack

    let result = searcher.skip_loop(haystack, window_end, backstop);
    assert_eq!(result, Some(3)); // should return index of 'a'
}

#[test]
fn test_skip_loop_not_found() {
    let pattern = b"abc".to_vec();
    let searcher = BoyerMooreSearch::new(pattern);
    
    let haystack = b"zzzxyz";
    let window_end = 5; // 'z' in the string "zzzxyz"
    let backstop = 6; // length of the haystack

    let result = searcher.skip_loop(haystack, window_end, backstop);
    assert_eq!(result, None); // pattern does not exist in haystack
}

#[test]
fn test_skip_loop_reaches_backstop() {
    let pattern = b"abc".to_vec();
    let searcher = BoyerMooreSearch::new(pattern);
    
    let haystack = b"abcxyz";
    let window_end = 3; // pointing to 'x'
    let backstop = 4; // just before the end of the haystack

    let result = searcher.skip_loop(haystack, window_end, backstop);
    assert_eq!(result, Some(4)); // should return index of 'y'
}

