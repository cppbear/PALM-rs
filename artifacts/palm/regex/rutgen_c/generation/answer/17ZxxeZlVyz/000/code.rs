// Answer 0

#[test]
fn test_boyer_moore_search_find_basic() {
    let pattern = b"abc".to_vec();
    let bms = BoyerMooreSearch::new(pattern.clone());
    let haystack = b"xyzabcdef";

    assert_eq!(bms.find(haystack), Some(3));
}

#[test]
fn test_boyer_moore_search_find_no_match() {
    let pattern = b"abc".to_vec();
    let bms = BoyerMooreSearch::new(pattern.clone());
    let haystack = b"xyzdef";

    assert_eq!(bms.find(haystack), None);
}

#[test]
fn test_boyer_moore_search_find_empty_haystack() {
    let pattern = b"abc".to_vec();
    let bms = BoyerMooreSearch::new(pattern.clone());
    let haystack = b"";

    assert_eq!(bms.find(haystack), None);
}

#[test]
fn test_boyer_moore_search_find_pattern_larger_than_haystack() {
    let pattern = b"abcd".to_vec();
    let bms = BoyerMooreSearch::new(pattern.clone());
    let haystack = b"abc";

    assert_eq!(bms.find(haystack), None);
}

#[test]
fn test_boyer_moore_search_find_multiple_occurrences() {
    let pattern = b"abc".to_vec();
    let bms = BoyerMooreSearch::new(pattern.clone());
    let haystack = b"xyzabcabc";

    assert_eq!(bms.find(haystack), Some(3));
}

#[test]
fn test_boyer_moore_search_find_shifted_matches() {
    let pattern = b"abc".to_vec();
    let bms = BoyerMooreSearch::new(pattern.clone());
    let haystack = b"xyzabczabc";

    assert_eq!(bms.find(haystack), Some(5));
}

