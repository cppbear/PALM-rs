// Answer 0

#[test]
fn test_is_suffix_shorter_text_length() {
    let pattern = vec![1, 2, 3];
    let freqy_packed = FreqyPacked::new(pattern.clone());
    
    let text_short = vec![1, 2]; // text length (2) < pattern length (3)
    assert_eq!(freqy_packed.is_suffix(&text_short), false);
}

#[test]
fn test_is_suffix_empty_text() {
    let pattern = vec![1, 2, 3];
    let freqy_packed = FreqyPacked::new(pattern.clone());
    
    let empty_text = vec![]; // text length (0) < pattern length (3)
    assert_eq!(freqy_packed.is_suffix(&empty_text), false);
}

#[test]
fn test_is_suffix_one_character_text() {
    let pattern = vec![1, 2, 3];
    let freqy_packed = FreqyPacked::new(pattern.clone());
    
    let one_char_text = vec![1]; // text length (1) < pattern length (3)
    assert_eq!(freqy_packed.is_suffix(&one_char_text), false);
}

