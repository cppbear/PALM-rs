// Answer 0

#[test]
fn test_check_match_guard_mismatch() {
    let pattern = b"abc".to_vec();
    let search = BoyerMooreSearch::new(pattern);
    let haystack = b"xyz"; // haystack does not contain the guard character
    let window_end = 2; // valid window_end
    let result = search.check_match(haystack, window_end);
}

#[test]
fn test_check_match_with_short_haystack() {
    let pattern = b"ab".to_vec();
    let search = BoyerMooreSearch::new(pattern);
    let haystack = b"x"; // haystack shorter than the pattern
    let window_end = 1; // valid window_end
    let result = search.check_match(haystack, window_end);
}

#[test]
fn test_check_match_guard_at_end() {
    let pattern = b"abc".to_vec();
    let search = BoyerMooreSearch::new(pattern);
    let haystack = b"abz"; // guard does not match ('z' instead of 'c')
    let window_end = 3; // valid window_end 
    let result = search.check_match(haystack, window_end);
}

#[test]
fn test_check_match_guard_in_middle() {
    let pattern = b"abc".to_vec();
    let search = BoyerMooreSearch::new(pattern);
    let haystack = b"zag"; // guard at the window_end - guard_reverse_idx does not match
    let window_end = 3; // valid window_end 
    let result = search.check_match(haystack, window_end);
}

#[test]
fn test_check_match_guard_mismatch_large_haystack() {
    let pattern = b"abc".to_vec();
    let search = BoyerMooreSearch::new(pattern);
    let haystack = b"xyzxyzxyzxyzxyzxyz"; // large haystack
    let window_end = 20; // valid window_end 
    let result = search.check_match(haystack, window_end);
}

