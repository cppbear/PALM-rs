// Answer 0

#[test]
fn test_find_pattern_found() {
    let pattern = vec![b'a', b'b', b'a'];
    let mut freqy_packed = FreqyPacked {
        pat: pattern.clone(),
        char_len: 3,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };

    let haystack = b"xxabaxyz";
    assert_eq!(freqy_packed.find(haystack), Some(2));
}

#[test]
fn test_find_pattern_not_found() {
    let pattern = vec![b'a', b'b', b'a'];
    let freqy_packed = FreqyPacked {
        pat: pattern.clone(),
        char_len: 3,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };

    let haystack = b"xxxyzz";
    assert_eq!(freqy_packed.find(haystack), None);
}

#[test]
fn test_find_empty_haystack() {
    let pattern = vec![b'a', b'b', b'a'];
    let freqy_packed = FreqyPacked {
        pat: pattern.clone(),
        char_len: 3,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };

    let haystack = b"";
    assert_eq!(freqy_packed.find(haystack), None);
}

#[test]
fn test_find_haystack_shorter_than_pattern() {
    let pattern = vec![b'a', b'b', b'a'];
    let freqy_packed = FreqyPacked {
        pat: pattern.clone(),
        char_len: 3,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };

    let haystack = b"ab";
    assert_eq!(freqy_packed.find(haystack), None);
}

#[test]
fn test_find_pattern_with_incomplete_utf8() {
    let pattern = vec![b'a', b'b', b'\xC2']; // Incomplete UTF-8 character
    let freqy_packed = FreqyPacked {
        pat: pattern.clone(),
        char_len: 3,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };

    let haystack = b"xxabaxyz";
    assert_eq!(freqy_packed.find(haystack), Some(2));
}

