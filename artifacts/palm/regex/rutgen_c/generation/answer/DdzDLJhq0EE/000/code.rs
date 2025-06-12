// Answer 0

#[test]
fn test_new_empty_pattern() {
    let pattern: Vec<u8> = vec![];
    let freqy_packed = FreqyPacked::new(pattern);
    assert_eq!(freqy_packed.len(), 0);
    assert_eq!(freqy_packed.char_len(), 0);
}

#[test]
fn test_new_single_byte_pattern() {
    let pattern = vec![b'a'];
    let freqy_packed = FreqyPacked::new(pattern);
    assert_eq!(freqy_packed.len(), 1);
    assert_eq!(freqy_packed.char_len(), 1);
    assert_eq!(freqy_packed.rare1, b'a');
    assert_eq!(freqy_packed.rare2, b'a');
    assert_eq!(freqy_packed.rare1i, 0);
    assert_eq!(freqy_packed.rare2i, 0);
}

#[test]
fn test_new_multiple_bytes_with_unique_rares() {
    let pattern = vec![b'a', b'b', b'c', b'd', b'e', b'f'];
    let freqy_packed = FreqyPacked::new(pattern);
    assert_eq!(freqy_packed.len(), 6);
    assert_eq!(freqy_packed.char_len(), 6);
    assert_eq!(freqy_packed.rare1, b'a');
    assert_eq!(freqy_packed.rare2, b'b');
    assert_eq!(freqy_packed.rare1i, 0);
    assert_eq!(freqy_packed.rare2i, 1);
}

#[test]
fn test_new_multiple_bytes_with_repeated_rares() {
    let pattern = vec![b'a', b'b', b'a', b'c', b'b'];
    let freqy_packed = FreqyPacked::new(pattern);
    assert_eq!(freqy_packed.len(), 5);
    assert_eq!(freqy_packed.char_len(), 5);
    assert_eq!(freqy_packed.rare1, b'a');
    assert_eq!(freqy_packed.rare2, b'b');
    assert_eq!(freqy_packed.rare1i, 2);
    assert_eq!(freqy_packed.rare2i, 4);
}

#[test]
fn test_new_complex_pattern() {
    let pattern = vec![b'z', b'y', b'x', b'y', b'z', b'x', b'a'];
    let freqy_packed = FreqyPacked::new(pattern);
    assert_eq!(freqy_packed.len(), 7);
    assert_eq!(freqy_packed.char_len(), 7);
    assert_eq!(freqy_packed.rare1, b'a');
    assert_eq!(freqy_packed.rare2, b'z');
    assert_eq!(freqy_packed.rare1i, 6);
    assert_eq!(freqy_packed.rare2i, 4);
}

