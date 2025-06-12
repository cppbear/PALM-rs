// Answer 0

#[test]
fn test_find_exact_match() {
    let pattern = b"abc".to_vec();
    let haystack = b"abc".to_vec();
    let boyer_moore = BoyerMooreSearch::new(pattern.clone());

    assert_eq!(boyer_moore.find(&haystack), Some(0));
}

#[test]
fn test_find_short_circuit() {
    let pattern = b"abc".to_vec();
    let haystack = b"abcdabcdabcdabcdabc".to_vec();
    let boyer_moore = BoyerMooreSearch::new(pattern.clone());

    assert_eq!(boyer_moore.find(&haystack), Some(0));
}

#[test]
fn test_find_with_skip() {
    let pattern = b"abc".to_vec();
    let haystack = b"xxaxyzabcxyz".to_vec();
    let boyer_moore = BoyerMooreSearch::new(pattern.clone());

    assert_eq!(boyer_moore.find(&haystack), Some(5));
}

#[test]
fn test_find_no_match() {
    let pattern = b"abc".to_vec();
    let haystack = b"defghijkl".to_vec();
    let boyer_moore = BoyerMooreSearch::new(pattern.clone());

    assert_eq!(boyer_moore.find(&haystack), None);
}

#[test]
fn test_find_multiple_occurrences() {
    let pattern = b"abc".to_vec();
    let haystack = b"abcabcabcabc".to_vec();
    let boyer_moore = BoyerMooreSearch::new(pattern.clone());

    assert_eq!(boyer_moore.find(&haystack), Some(0));
}

