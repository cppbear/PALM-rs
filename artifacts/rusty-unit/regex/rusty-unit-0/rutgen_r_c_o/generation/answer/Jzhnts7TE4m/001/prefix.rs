// Answer 0

#[test]
fn test_len_empty() {
    let freqy = FreqyPacked::empty();
    freqy.len();
}

#[test]
fn test_len_single_element() {
    let freqy = FreqyPacked::new(vec![0]);
    freqy.len();
}

#[test]
fn test_len_two_elements() {
    let freqy = FreqyPacked::new(vec![1, 2]);
    freqy.len();
}

#[test]
fn test_len_large_vector() {
    let freqy = FreqyPacked::new(vec![0; 2^30]);
    freqy.len();
}

