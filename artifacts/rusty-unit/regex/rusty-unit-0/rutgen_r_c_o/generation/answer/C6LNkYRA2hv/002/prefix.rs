// Answer 0

#[test]
fn test_is_suffix_equal_length_unique_bytes() {
    let pat = vec![1, 2, 3, 4, 5];
    let text = vec![1, 2, 3, 4, 5];
    let freqy = FreqyPacked::new(pat);
    freqy.is_suffix(&text);
}

#[test]
fn test_is_suffix_short_text_same_pattern() {
    let pat = vec![97]; // ASCII for 'a'
    let text = vec![97]; // same as pattern
    let freqy = FreqyPacked::new(pat);
    freqy.is_suffix(&text);
}

#[test]
fn test_is_suffix_long_pattern() {
    let pat = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let text = pat.clone(); // equal length and content
    let freqy = FreqyPacked::new(pat);
    freqy.is_suffix(&text);
}

#[test]
fn test_is_suffix_non_matching_unique_bytes() {
    let pat = vec![1, 2, 3, 4];
    let text = vec![4, 3, 2, 1]; // different content
    let freqy = FreqyPacked::new(pat);
    freqy.is_suffix(&text);
}

#[test]
fn test_is_suffix_with_bound_length() {
    let pat = (1..=1024).map(|x| x as u8).collect::<Vec<u8>>();
    let text = pat.clone(); // equal length and content
    let freqy = FreqyPacked::new(pat);
    freqy.is_suffix(&text);
}

#[test]
fn test_is_suffix_edge_case_empty() {
    let pat = vec![1];
    let text = vec![1]; // same as pattern
    let freqy = FreqyPacked::new(pat);
    freqy.is_suffix(&text);
}

#[test]
fn test_is_suffix_maximum_length() {
    let pat = (1..=1024).map(|x| x as u8).collect::<Vec<u8>>();
    let text = pat.clone(); // equal length and content
    let freqy = FreqyPacked::new(pat);
    freqy.is_suffix(&text);
}

