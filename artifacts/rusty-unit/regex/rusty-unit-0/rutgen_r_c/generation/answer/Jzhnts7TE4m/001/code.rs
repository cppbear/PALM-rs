// Answer 0

#[test]
fn test_len_non_empty_pattern() {
    let pattern = vec![b'a', b'b', b'c'];
    let freqy = FreqyPacked::new(pattern);
    assert_eq!(freqy.len(), 3);
}

#[test]
fn test_len_empty_pattern() {
    let pattern = Vec::new();
    let freqy = FreqyPacked::new(pattern);
    assert_eq!(freqy.len(), 0);
}

#[test]
fn test_len_single_byte_pattern() {
    let pattern = vec![b'x'];
    let freqy = FreqyPacked::new(pattern);
    assert_eq!(freqy.len(), 1);
}

#[test]
fn test_len_large_pattern() {
    let pattern = vec![b'a'; 1024]; // A pattern of 1024 'a' bytes
    let freqy = FreqyPacked::new(pattern);
    assert_eq!(freqy.len(), 1024);
}

