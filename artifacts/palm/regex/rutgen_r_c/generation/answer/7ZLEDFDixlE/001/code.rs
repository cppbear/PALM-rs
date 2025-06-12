// Answer 0

#[test]
fn test_compile_skip_table_with_non_repeating_pattern() {
    let pattern: Vec<u8> = b"abc".to_vec();
    let expected: Vec<usize> = vec![3, 2, 1, 3]; // 3 is for sentinel
    
    let result = compile_skip_table(&pattern);

    assert_eq!(result, expected);
}

#[test]
fn test_compile_skip_table_with_repeating_pattern() {
    let pattern: Vec<u8> = b"aab".to_vec();
    let expected: Vec<usize> = vec![3, 1, 2, 3]; // 3 is for sentinel

    let result = compile_skip_table(&pattern);

    assert_eq!(result, expected);
}

#[test]
fn test_compile_skip_table_with_empty_pattern() {
    let pattern: Vec<u8> = Vec::new();
    let expected: Vec<usize> = vec![0; 256]; // all values should be zero since the pattern is empty

    let result = compile_skip_table(&pattern);

    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn test_compile_skip_table_with_invalid_byte() {
    let pattern: Vec<u8> = vec![250, 251, 252, 253, 254, 255]; // using values that might exceed ranges

    // This test should panic due to indexing out of bounds in the skip table
    compile_skip_table(&pattern);
}

