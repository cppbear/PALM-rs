// Answer 0

#[test]
fn test_find_exact_match() {
    let pattern = b"test";
    let haystack = b"This is a test string for testing purposes. test";
    let searcher = BoyerMooreSearch::new(pattern.to_vec());

    let result = searcher.find(haystack);
    assert_eq!(result, Some(10)); // The start of the first occurrence of "test" in the haystack
}

#[test]
fn test_find_shorter_haystack() {
    let pattern = b"long";
    let haystack = b"short";
    let searcher = BoyerMooreSearch::new(pattern.to_vec());

    let result = searcher.find(haystack);
    assert_eq!(result, None); // Haystack is shorter than the pattern
}

#[test]
fn test_find_non_matching_pattern() {
    let pattern = b"hello";
    let haystack = b"This is a simple test.";
    let searcher = BoyerMooreSearch::new(pattern.to_vec());

    let result = searcher.find(haystack);
    assert_eq!(result, None); // Pattern does not match
}

#[test]
fn test_find_success_after_skip_loop() {
    let pattern = b"abc";
    let haystack = b"xxabcxx";
    let searcher = BoyerMooreSearch::new(pattern.to_vec());

    let result = searcher.find(haystack);
    assert_eq!(result, Some(2)); // The start of the first occurrence of "abc"
}

#[test]
fn test_find_with_sufficient_length_haystack() {
    let pattern = b"pattern";
    let haystack = b"This is a long haystack containing the word pattern somewhere.";
    let searcher = BoyerMooreSearch::new(pattern.to_vec());

    let result = searcher.find(haystack);
    assert_eq!(result, Some(34)); // The start of the first occurrence of "pattern"
}

