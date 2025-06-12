// Answer 0

#[test]
fn test_find_with_haystack_equal_to_pattern_length_and_non_empty_pattern() {
    let pattern = vec![1, 2, 3, 4, 5];
    let freqy_packed = FreqyPacked {
        pat: pattern.clone(),
        char_len: pattern.len(),
        rare1: pattern[0],
        rare1i: 0,
        rare2: pattern[1],
        rare2i: 1,
    };
    let haystack = vec![1, 2, 3, 4, 5];
    let result = freqy_packed.find(&haystack);
    assert_eq!(result, Some(0));
}

#[test]
fn test_find_with_haystack_shorter_than_pattern_length() {
    let pattern = vec![1, 2, 3, 4, 5];
    let freqy_packed = FreqyPacked {
        pat: pattern.clone(),
        char_len: pattern.len(),
        rare1: pattern[0],
        rare1i: 0,
        rare2: pattern[1],
        rare2i: 1,
    };
    let haystack = vec![1, 2, 3];
    let result = freqy_packed.find(&haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_with_empty_pattern() {
    let pattern = Vec::<u8>::new();
    let freqy_packed = FreqyPacked {
        pat: pattern,
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };
    let haystack = vec![1, 2, 3, 4, 5];
    let result = freqy_packed.find(&haystack);
    assert_eq!(result, None);
}

