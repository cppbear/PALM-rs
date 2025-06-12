// Answer 0

#[test]
fn test_check_match_valid_case() {
    let pattern = b"abc".to_vec();
    let haystack = b"xyzabc".to_vec();
    let window_end = 5; // points to the last character 'c' in "xyzabc"
    
    let bms = BoyerMooreSearch::new(pattern.clone());
    
    assert_eq!(bms.check_match(&haystack, window_end), true);
}

#[test]
fn test_check_match_guard_fail() {
    let pattern = b"abc".to_vec();
    let haystack = b"xyzabc".to_vec();
    let window_end = 6; // out of bounds; haystack[window_end - 1] should equal 'c', but it will access haystack[5]
    
    let bms = BoyerMooreSearch::new(pattern.clone());

    assert_eq!(bms.check_match(&haystack, window_end), false);
}

#[test]
fn test_check_match_no_match() {
    let pattern = b"abc".to_vec();
    let haystack = b"xyzdef".to_vec();
    let window_end = 6; // haystack ends; no match
    
    let bms = BoyerMooreSearch::new(pattern.clone());

    assert_eq!(bms.check_match(&haystack, window_end), false);
}

#[test]
fn test_check_match_empty_pattern() {
    let pattern = b"".to_vec();
    let haystack = b"xyzabc".to_vec();
    let window_end = 3; // Ensure we don't panic on empty pattern
    
    let bms = BoyerMooreSearch::new(pattern.clone());

    assert_eq!(bms.check_match(&haystack, window_end), false);
}

