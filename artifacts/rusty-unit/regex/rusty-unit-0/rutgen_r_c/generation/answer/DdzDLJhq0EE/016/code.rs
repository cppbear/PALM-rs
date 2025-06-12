// Answer 0

#[test]
fn test_new_function_empty_pattern() {
    let pat: Vec<u8> = vec![];
    let result = FreqyPacked::new(pat);
    assert_eq!(result.pat.len(), 0);
    assert_eq!(result.char_len, 0);
    assert_eq!(result.rare1, 0);
    assert_eq!(result.rare1i, 0);
    assert_eq!(result.rare2, 0);
    assert_eq!(result.rare2i, 0);
}

#[test]
fn test_new_function_single_byte_pattern() {
    let pat: Vec<u8> = vec![97]; // 'a'
    let result = FreqyPacked::new(pat);
    assert_eq!(result.pat.len(), 1);
    assert_eq!(result.char_len, 1);
    assert_eq!(result.rare1, 97);
    assert_eq!(result.rare1i, 0);
    assert_eq!(result.rare2, 97);
    assert_eq!(result.rare2i, 0);
}

#[test]
fn test_new_function_two_bytes_pattern() {
    let pat: Vec<u8> = vec![97, 98]; // 'a', 'b'
    let result = FreqyPacked::new(pat);
    assert_eq!(result.pat.len(), 2);
    assert_eq!(result.char_len, 2);
    assert_eq!(result.rare1, 97);
    assert_eq!(result.rare1i, 0);
    assert_eq!(result.rare2, 98);
    assert_eq!(result.rare2i, 1);
}

#[test]
fn test_new_function_repeated_bytes_pattern() {
    let pat: Vec<u8> = vec![97, 97, 98, 98]; // 'a', 'a', 'b', 'b'
    let result = FreqyPacked::new(pat);
    assert_eq!(result.pat.len(), 4);
    assert_eq!(result.char_len, 4);
    assert_eq!(result.rare1, 97);
    assert_eq!(result.rare1i, 1);
    assert_eq!(result.rare2, 98);
    assert_eq!(result.rare2i, 3);
}

#[test]
fn test_new_function_distinct_bytes_pattern() {
    let pat: Vec<u8> = vec![100, 101, 102]; // 'd', 'e', 'f'
    let result = FreqyPacked::new(pat);
    assert_eq!(result.pat.len(), 3);
    assert_eq!(result.char_len, 3);
    assert_eq!(result.rare1, 100);
    assert_eq!(result.rare1i, 0);
    assert_eq!(result.rare2, 101);
    assert_eq!(result.rare2i, 1);
}

#[test]
fn test_new_function_with_higher_bytes_pattern() {
    let pat: Vec<u8> = vec![200, 300, 100]; // high values for testing
    let result = FreqyPacked::new(pat);
    assert_eq!(result.pat.len(), 3);
    assert_eq!(result.char_len, 3);
    assert_eq!(result.rare1, 100);
    assert_eq!(result.rare1i, 2);
    assert_eq!(result.rare2, 200);
    assert_eq!(result.rare2i, 0);
}

