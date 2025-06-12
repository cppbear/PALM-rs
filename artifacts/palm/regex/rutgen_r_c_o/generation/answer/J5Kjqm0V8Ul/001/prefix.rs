// Answer 0

#[test]
fn test_find_haystack_shorter_than_pattern() {
    let pattern = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // length 10
    let haystack = vec![1, 2, 3, 4, 5]; // length 5, so haystack < pattern
    let freqy_packed = FreqyPacked::new(pattern);
    let result = freqy_packed.find(&haystack);
}

#[test]
fn test_find_haystack_empty_pattern_long() {
    let pattern = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]; // length 15
    let haystack = vec![]; // length 0
    let freqy_packed = FreqyPacked::new(pattern);
    let result = freqy_packed.find(&haystack);
}

#[test]
fn test_find_haystack_length_nine_pattern_length_ten() {
    let pattern = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // length 10
    let haystack = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]; // length 9
    let freqy_packed = FreqyPacked::new(pattern);
    let result = freqy_packed.find(&haystack);
}

#[test]
fn test_find_haystack_length_seven_pattern_length_twenty() {
    let pattern = vec![1; 20]; // length 20
    let haystack = vec![1; 7]; // length 7
    let freqy_packed = FreqyPacked::new(pattern);
    let result = freqy_packed.find(&haystack);
}

#[test]
fn test_find_haystack_length_eight_pattern_length_twelve() {
    let pattern = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]; // length 12
    let haystack = vec![1, 2, 3, 4, 5, 6, 7, 8]; // length 8
    let freqy_packed = FreqyPacked::new(pattern);
    let result = freqy_packed.find(&haystack);
}

