// Answer 0

#[test]
fn test_compile_skip_table_empty_pattern() {
    let pattern: Vec<u8> = vec![];
    let result = compile_skip_table(&pattern);
    assert_eq!(result.len(), 256);
    assert!(result.iter().all(|&x| x == pattern.len()));
}

#[test]
fn test_compile_skip_table_single_character_pattern() {
    let pattern: Vec<u8> = vec![b'a'];
    let result = compile_skip_table(&pattern);
    assert_eq!(result.len(), 256);
    assert_eq!(result[b'a' as usize], 0);
    assert!(result.iter().filter(|&&x| x == 0).count() == 1);
}

#[test]
fn test_compile_skip_table_multiple_characters_pattern() {
    let pattern: Vec<u8> = vec![b'a', b'b', b'c', b'a'];
    let result = compile_skip_table(&pattern);
    assert_eq!(result.len(), 256);
    assert_eq!(result[b'a' as usize], 1);
    assert_eq!(result[b'b' as usize], 2);
    assert_eq!(result[b'c' as usize], 3);
}

#[test]
fn test_compile_skip_table_full_character_set() {
    let pattern: Vec<u8> = (0..256).map(|x| x as u8).collect();
    let result = compile_skip_table(&pattern);
    assert_eq!(result.len(), 256);
    for (i, &c) in pattern.iter().enumerate() {
        assert_eq!(result[c as usize], pattern.len() - 1 - i);
    }
}

