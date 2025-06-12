// Answer 0

#[test]
fn test_find_empty_pattern_and_haystack() {
    let pat = Vec::new(); // empty pattern
    let haystack = b""; // empty haystack

    let freqy_packed = FreqyPacked::new(pat);
    let result = freqy_packed.find(haystack);
}

#[test]
fn test_find_pattern_longer_than_haystack() {
    let pat = b"abcd".to_vec(); // pattern of length 4
    let haystack = b"abc".to_vec(); // haystack of length 3

    let freqy_packed = FreqyPacked::new(pat);
    let result = freqy_packed.find(&haystack);
} 

#[test]
fn test_find_haystack_equals_pattern() {
    let pat = b"abcd".to_vec(); // pattern of length 4
    let haystack = b"abcd".to_vec(); // haystack of length 4

    let freqy_packed = FreqyPacked::new(pat);
    let result = freqy_packed.find(&haystack);
}

#[test]
fn test_find_haystack_not_containing_pattern() {
    let pat = b"abcd".to_vec(); // pattern of length 4
    let haystack = b"abef".to_vec(); // haystack of length 4

    let freqy_packed = FreqyPacked::new(pat);
    let result = freqy_packed.find(&haystack);
}

