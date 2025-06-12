// Answer 0

#[test]
fn test_find_haystack_shorter_than_pattern() {
    let pattern = b"longer pattern";
    let haystack = b"short";
    
    let searcher = BoyerMooreSearch::new(pattern.to_vec());
    let result = searcher.find(haystack);
    
    assert_eq!(result, None);
}

#[test]
fn test_find_haystack_empty_with_non_empty_pattern() {
    let pattern = b"pattern";
    let haystack = b"";
    
    let searcher = BoyerMooreSearch::new(pattern.to_vec());
    let result = searcher.find(haystack);
    
    assert_eq!(result, None);
}

#[test]
fn test_find_haystack_single_byte_with_multibyte_pattern() {
    let pattern = b"multi";
    let haystack = b"a";
    
    let searcher = BoyerMooreSearch::new(pattern.to_vec());
    let result = searcher.find(haystack);
    
    assert_eq!(result, None);
}

#[test]
fn test_find_haystack_exactly_equal_to_pattern() {
    let pattern = b"match";
    let haystack = b"match";
    
    let searcher = BoyerMooreSearch::new(pattern.to_vec());
    let result = searcher.find(haystack);
    
    assert_eq!(result, None);
}

