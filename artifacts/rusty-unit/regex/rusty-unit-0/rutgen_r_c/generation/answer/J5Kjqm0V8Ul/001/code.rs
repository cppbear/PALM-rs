// Answer 0

#[test]
fn test_find_haystack_shorter_than_pattern() {
    let pattern = vec![b'a', b'b', b'c'];
    let freqy_packed = FreqyPacked {
        pat: pattern.clone(),
        char_len: pattern.len(),
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    let haystack = vec![b'a']; // haystack is shorter than the pattern
    let result = freqy_packed.find(&haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_empty_pattern() {
    let pattern = vec![];
    let freqy_packed = FreqyPacked {
        pat: pattern.clone(),
        char_len: 0,
        rare1: 0,
        rare1i: 0,
        rare2: 0,
        rare2i: 0,
    };
    let haystack = vec![b'a', b'b', b'c'];
    let result = freqy_packed.find(&haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_haystack_equal_to_pattern() {
    let pattern = vec![b'a', b'b', b'c'];
    let freqy_packed = FreqyPacked {
        pat: pattern.clone(),
        char_len: pattern.len(),
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    let haystack = vec![b'a', b'b']; // haystack is shorter than the pattern
    let result = freqy_packed.find(&haystack);
    assert_eq!(result, None);
}

