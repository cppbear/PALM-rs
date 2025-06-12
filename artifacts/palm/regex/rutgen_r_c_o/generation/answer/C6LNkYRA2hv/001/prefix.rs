// Answer 0

#[test]
fn test_is_suffix_short_haystack() {
    let pattern = vec![1, 2, 3]; // pattern length is 3
    let freqy = FreqyPacked::new(pattern);
    let haystack = vec![4, 5]; // haystack length is 2, less than pattern length
    let result = freqy.is_suffix(&haystack);
}

#[test]
fn test_is_suffix_exactly_short_haystack() {
    let pattern = vec![10, 20]; // pattern length is 2
    let freqy = FreqyPacked::new(pattern);
    let haystack = vec![30]; // haystack length is 1, less than pattern length
    let result = freqy.is_suffix(&haystack);
}

#[test]
fn test_is_suffix_empty_haystack() {
    let pattern = vec![7]; // pattern length is 1
    let freqy = FreqyPacked::new(pattern);
    let haystack = vec![]; // haystack length is 0, less than pattern length
    let result = freqy.is_suffix(&haystack);
}

#[test]
fn test_is_suffix_large_haystack() {
    let pattern = vec![9, 8, 7, 6, 5]; // pattern length is 5
    let freqy = FreqyPacked::new(pattern);
    let haystack = vec![0; 4]; // haystack length is 4, less than pattern length
    let result = freqy.is_suffix(&haystack);
}

#[test]
fn test_is_suffix_another_short_haystack() {
    let pattern = vec![100, 101, 102, 103]; // pattern length is 4
    let freqy = FreqyPacked::new(pattern);
    let haystack = vec![200, 201]; // haystack length is 2, less than pattern length
    let result = freqy.is_suffix(&haystack);
}

