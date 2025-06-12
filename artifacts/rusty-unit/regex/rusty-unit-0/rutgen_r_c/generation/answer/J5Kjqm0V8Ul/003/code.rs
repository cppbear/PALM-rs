// Answer 0

#[test]
fn test_find_haystack_equal_to_pattern_length() {
    let pattern = vec![b'a', b'b', b'c'];
    let haystack = vec![b'a', b'b', b'c']; // haystack.len() == pat.len()
    let freqy = FreqyPacked {
        pat: pattern.clone(),
        char_len: pattern.len(),
        rare1: pattern[0],
        rare1i: 0,
        rare2: pattern[1],
        rare2i: 1,
    };
    
    let result = freqy.find(&haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_haystack_with_non_matching_pattern() {
    let pattern = vec![b'a', b'b', b'c'];
    let haystack = vec![b'd', b'e', b'f']; // haystack.len() == pat.len() but does not match
    let freqy = FreqyPacked {
        pat: pattern.clone(),
        char_len: pattern.len(),
        rare1: pattern[0],
        rare1i: 0,
        rare2: pattern[1],
        rare2i: 1,
    };

    let result = freqy.find(&haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_haystack_with_empty_pattern_should_return_none() {
    let pattern: Vec<u8> = vec![]; // empty pattern
    let haystack = vec![b'a', b'b', b'c'];
    
    let freqy = FreqyPacked {
        pat: pattern,
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };

    let result = freqy.find(&haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_haystack_with_pattern_length_zero() {
    let pattern = vec![b'a', b'b', b'c'];
    let haystack = vec![b'a', b'b', b'c'];
    let freqy = FreqyPacked {
        pat: pattern.clone(),
        char_len: pattern.len(),
        rare1: pattern[0],
        rare1i: 0,
        rare2: pattern[1],
        rare2i: 1,
    };
    
    let result = freqy.find(&haystack);
    assert_eq!(result, None);
}

